# Rust Kratos Web

This sandbox project is to test how to use Rust Kratos on an HTML page.

## Status

WIP

## Getting Started

Since this is a web frontend project, the following needs to be installed to be
able to start this project

```bash
cargo install trunk
cargo install leptosfmt # For formatting the leptos view! macro
rustup target add wasm32-unknown-unknown
```

To run

```bash
# In the first terminal window
docker-compose up

# In another terminal window
trunk serve
    # access the page at http://localhost:8080
```
