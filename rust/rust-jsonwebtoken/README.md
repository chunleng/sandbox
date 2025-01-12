# Rust JsonWebToken

This sandbox explores rust `jsonwebtoken` and JWT/JWK related crates.

## Status

WIP (Lack feature to encode EdDSA key, see note)

## Getting Started

```bash
cargo run
```

## Note

- We could use encode if we can translate the JWT into PEM key by using
  `jsonwebkey` crate. However, the crate currently does not support the
  conversion of EdDSA JWT to PEM key. And, with the effort needed to encode, I
  have decided to use [Ory Kratos](../rust-kratos-web) to perform that encoding
  task temporarily.
