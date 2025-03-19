# Rust Tracing OpenTelemetry

This sandbox project is to make sure tracing can be sent to OpenTelemetry
collector.

## Status

Working

## Getting Started

```bash
# Launch Grafana Alloy to receive the log and display what is received
docker-compose up

# Start app
cargo run
```

## Note

- Seems like the version difference changes things quite a lot for
  `opentelemetry-*` libraries.
- This example contains:
  * Setting of service name
  * Setting of a sample rate
