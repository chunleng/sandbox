# Rust Kratos Web

This sandbox project is to test how to use Rust Kratos on an HTML page.

## Status

Working

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

### Generating JWKS file

In order to generate the JWKS file, (`./docker/config/backend.jwk.json`) we can
use the following command:

```bash
docker run --rm -it oryd/oathkeeper credentials generate --alg EdDSA
```

## Notes

- [Jsonnet](./docker/config/backend.jsonnet) file is used to extend the claim
  you want to use in the JWT that you create for the user. It can be modified
  with values from below:

  ```jsonnet
  local claims = std.extVar('claims');
  local session = std.extVar('session');
  ```

## Reference

- Additional material on JWT creation:
  <https://reorchestrate.com/posts/custom-jwt-claims-with-ory-kratos/>
- Jsonnet <https://jsonnet.org/>
