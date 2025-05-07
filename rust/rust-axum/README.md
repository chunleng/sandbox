# Sandbox

Sandbox to try out various feature of axum.

## Status

Working

## Getting Started

### Streaming API

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

### Custom Validation

This one for adding a custom validation to the request object.

```bash
cargo run --examples=custom-validation
curl -i -X POST http://localhost:3000/kids_only -H "Content-Type: application/json" -d '{"age":12}'
```

### `utoipa` Implementation

For how to use the `utoipa` crate to generate OpenAPI specifications. There is
another example using `utoipa-axum` which autobinds the endpoint to utoipa

```bash
cargo run --examples=utoipa
cargo run --example=utoipa-axum

# To query:
curl -i http://localhost:3000/buy -H "Content-Type: application/json" -d '{"id":1,"name":"flower"}'
curl -i http://localhost:3000/check_item -H "Content-Type: application/json" -d '{"id":1,"name":"flower"}'

```
