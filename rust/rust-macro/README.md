# Rust Macro

This sandbox project is for me to test out some macros, and to understand how
Rust macro works.

## Status

Working

## Getting Started

To run,

```bash
cargo run
```

To debug the output, we can use the `cargo-expand` crate.

```bash
cargo expand -p app
```

## Note

- Did not manage to get the logic for compiling instruction right, and it does
  not work for `(x + y) - z` situation. But I am leaving it at that.
