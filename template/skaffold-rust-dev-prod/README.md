# Skaffold Rust

This sandbox project is to test out how to best optimize Rust for development in
Skaffold, as well as to deploy with the recommended settings

## Status

Working

## Getting Started

```bash
skaffold dev
skaffold run
```

## Notes

- In order to debug the distroless image, we can add `:debug` to it and run it
  with `--entrypoint=sh`. However, it seems that running without the entrypoint
  causes some issue with the program.
- Distroless static image does not seem to work at the time when I try to build
  it and this is because it seems like `/usr/lib/linux-gnu/libgcc_s.so.1` is
  needed for basic Rust to work, so I built it in distroless base image instead.
  To statically compile DLL into the app, one way is to use
  [`musl`](https://musl.libc.org/), but there are limitations in `musl` that we
  might want to consider.
