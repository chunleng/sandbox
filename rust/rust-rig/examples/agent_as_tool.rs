use anyhow::Result;
use rig_core::completion::{Prompt, ToolDefinition};
use rig_core::prelude::*;
use rig_core::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Download a GGUF model to ./model/qwen2.5-0.5b-instruct-q4.gguf before running.

#[derive(Deserialize)]
struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(Deserialize, Serialize)]
struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "add";
    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": { "type": "number", "description": "The first number to add" },
                    "y": { "type": "number", "description": "The second number to add" }
                },
                "required": ["x", "y"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("[tool-call] Adding {} and {}", args.x, args.y);
        Ok(args.x + args.y)
    }
}

#[derive(Deserialize, Serialize)]
struct Subtract;

impl Tool for Subtract {
    const NAME: &'static str = "subtract";
    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "subtract".to_string(),
            description: "Subtract y from x (i.e.: x - y)".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": { "type": "number", "description": "The number to subtract from" },
                    "y": { "type": "number", "description": "The number to subtract" }
                },
                "required": ["x", "y"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("[tool-call] Subtracting {} from {}", args.y, args.x);
        Ok(args.x - args.y)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            "warn,llama_cpp_2=off,ggml=off",
        ))
        .with_target(false)
        .init();

    let model_path = "./model/qwen2.5-0.5b-instruct-q4.gguf";

    let client = rig_llama_cpp::Client::builder(model_path)
        .n_ctx(8192)
        .build()?;

    // Agent B: calculator with arithmetic tools
    let calculator_agent = client
        .agent("local")
        .preamble("You are a calculator. Use the tools provided to perform arithmetic operations and answer the user's question.")
        .default_max_turns(100)
        .max_tokens(1024)
        .tool(Adder)
        .tool(Subtract)
        .build();

    // Agent A: assistant that delegates math to the calculator agent (Agent A calls Agent B)
    let agent_using_agent = client
        .agent("local")
        .preamble("You are a helpful assistant that can solve problems. Use the tool provided to answer the user's question.")
        .default_max_turns(100)
        .max_tokens(1024)
        .tool(calculator_agent)
        .build();

    println!("Calculate 2 - 5");
    println!(
        "Agent using agent: {}",
        agent_using_agent.prompt("Calculate 2 - 5").await?
    );

    Ok(())
}
