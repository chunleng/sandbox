use laya::{Agent, AgentBuilder};
use serde_json::json;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Downloads the checkpoint from the Hugging Face Hub on first use
    // (cached afterwards), then loads it on the best available device.
    let options = match candle_core::Device::new_metal(0) {
        Ok(device) => AgentBuilder::new().device(device),
        Err(_) => AgentBuilder::new(),
    };
    let start = Instant::now();
    let agent = Agent::from_pretrained_with("aac6fef/laya-mlx", None, None, options)?;
    let startup = start.elapsed();
    println!("device: {:?}", agent.device());
    println!("startup (load checkpoint): {startup:.3?}");

    // The state: plain text, or any JSON value (serialized compactly).
    let state = json!("I was billed twice for the same order. Please refund the duplicate charge.");

    // Questions keyed by id. Three kinds are supported:
    //   choice - pick one of the criteria labels
    //   score  - expected level over an ordered rubric
    //   noul   - probability that the statement holds
    let questions = json!({
        "department": {
            "type": "choice",
            "instructions": "Who should handle this?",
            "criteria": ["billing", "technical", "sales"]
        },
        "urgency": {
            "type": "score",
            "instructions": "How urgent is this request?",
            "criteria": ["routine", "same-day", "immediate"]
        },
        "refund_due": {
            "type": "noul",
            "instructions": "The customer is owed a refund."
        }
    });

    // One warm-up call: Metal shader compilation and buffer allocation happen
    // on the first forward pass, so it must not pollute the measurements.
    let result = agent.predict(&state, &questions)?;
    println!("\nfirst inference (includes warm-up): {:.3?}", {
        let start = Instant::now();
        agent.predict(&state, &questions)?;
        start.elapsed()
    });

    // 10 measured inferences.
    const RUNS: usize = 10;
    let mut timings = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        let start = Instant::now();
        agent.predict(&state, &questions)?;
        timings.push(start.elapsed());
    }
    timings.sort();

    let min = timings[0];
    let max = timings[RUNS - 1];
    let mean = timings.iter().sum::<std::time::Duration>() / RUNS as u32;
    let median = (timings[RUNS / 2 - 1] + timings[RUNS / 2]) / 2;
    let ms = |d: std::time::Duration| d.as_secs_f64() * 1000.0;
    println!("\n{RUNS} inferences (3 questions, batched in one forward pass):");
    println!("  min:    {:.1} ms", ms(min));
    println!("  median: {:.1} ms", ms(median));
    println!("  mean:   {:.1} ms", ms(mean));
    println!("  max:    {:.1} ms", ms(max));

    // Typed access: calibrated choice, expected rubric score, P(true).
    println!();
    for (id, answer) in &result.typed {
        match (answer.choice.as_deref(), answer.score, answer.noul) {
            (Some(choice), _, _) => println!("{id}: {choice} (confidence {:.2})", answer.confidence),
            (_, Some(score), _) => println!("{id}: {score:.2} (confidence {:.2})", answer.confidence),
            (_, _, Some(noul)) => println!("{id}: {noul:.2} (confidence {:.2})", answer.confidence),
            _ => {}
        }
    }

    // Full upstream-shaped JSON, if you need it downstream.
    println!("{}", serde_json::to_string_pretty(&result.to_value())?);

    Ok(())
}
