use std::error::Error;

use opentelemetry::trace::TracerProvider;
use otel::OtelHandler;
use tracing::{info, instrument, level_filters::LevelFilter, trace};
use tracing_subscriber::{fmt, prelude::*, registry};

mod otel;

#[instrument]
async fn add_to_trace(start: i32) {
    for value in start..(start + 200) {
        // Otel will stop at #127 for this, I assume it's due to the limitation 🤷
        trace!("Test trace. {}", value);
    }
    info!("Add to trace completed: {}", start);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let otel_handler = OtelHandler::init_tracer(0.05)?;
    let otel_layer =
        tracing_opentelemetry::layer().with_tracer(otel_handler.provider.tracer("lib"));
    let fmt_layer = fmt::layer().with_filter(LevelFilter::INFO);

    registry().with(otel_layer).with(fmt_layer).init();

    for value in 0..100 {
        add_to_trace(value).await;
    }

    otel_handler.shutdown_tracer()?;
    Ok(())
}
