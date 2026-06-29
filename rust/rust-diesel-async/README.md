# Rust Diesel Async

Testing how the `diesel-async` crate works.

## Status

Working

## Getting Started

```bash
# Startup a Postgres server
docker-compose up

# Run an example
cargo run --example basic
cargo run --example with-default
```

While the Postgres database is up, we can use the tools provided by `diesel-cli`
by using the following examples:

```bash
# Generate a migration
docker-compose run --rm -it diesel-migrate migration generate "initial_state"

# Revert a migration
docker-compose run --rm -it diesel-migrate migration revert
```
