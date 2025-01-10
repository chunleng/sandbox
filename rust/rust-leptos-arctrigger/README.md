# Rust Leptos ArcTrigger

This sandbox explores how to use ArcTrigger, as after release of Leptos 0.7, it
is stated that [storing of Views on signal is an
antipattern](https://github.com/leptos-rs/leptos/releases/tag/v0.7.0) and would
no longer be allowed to compile.

## Status

Working

## Getting Started

```bash
cargo install trunk
cargo install leptosfmt # For formatting the leptos view! macro
rustup target add wasm32-unknown-unknown
```

To run

```bash
trunk serve
```
