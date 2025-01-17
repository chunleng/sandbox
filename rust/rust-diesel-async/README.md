# Rust Diesel Async

Testing how the `diesel-async` crate works.

## Status

WIP

## Getting Started

```bash
# Startup a Postgres server
docker-compose up

# Run the code
cargo run
```

While the Postgres database is up, we can use the tools provided by `diesel-cli`
by using the following examples:

```bash
# Generate a migration
docker-compose run --rm -it diesel-migrate migration generate "initial_state"

# Revert a migration
docker-compose run --rm -it diesel-migrate migration revert
```
