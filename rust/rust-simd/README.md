# SIMD

This project is to understand how SIMD in Rust works. It shows test on how SIMD
works against non-SIMD run in terms of performance

## Status

Working

## Getting Started

```bash
# Run bench (SIMD)
cargo +nightly bench --features=simd

# Run bench (scalar)
cargo bench
```

## Clearing Benchmark Cache

Criterion compares against the previous run by default. To start fresh:

```bash
rm -rf target/criterion
```

## Notes

- `rustc`'s `opt-level` is set to 1 to prevent auto-vectorization of the scalar
  add function
