# Skaffold Rust

This sandbox project is to test out how to best optimize Rust with multiple
workspace for development in Skaffold, as well as to deploy with the recommended
settings

## Status

Working

## Getting Started

```bash
skaffold dev
skaffold run
```

## Notes

- This project is copied from
  [skaffold-rust-dev-prod](../skaffold-rust-dev-prod), so make sure to check the
  notes over there too.
- For Dev, all the deployments are overridden to use the same container, only
  having difference of launching with different command. Skaffold sync is done
  for all workspace code and `Cargo.toml`. However, we do not sync the root
  `Cargo.toml` as it provides information about the workspace which, when
  changed should invoke Docker rebuild.
