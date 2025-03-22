# Sandbox

Sandbox to try out various feature of axum.

## Status

Working

## Getting Started

This tries out how streaming work in axum and you can try out using the
following code.

```bash
cargo run --examples=stream

curl -N -i "localhost:3000/stream_numbers?end=10"
curl -N --http2-prior-knowledge -i "localhost:3000/stream_numbers?end=10"
curl -N -i localhost:3000/stream_names
curl -N --http2-prior-knowledge -i localhost:3000/stream_names
```

The above demo both way of using a streaming library and using tokio channel to
achieve the similar operations.
