# Kubernetes Secret File

This sandbox project is used to test attachment of secrets to the container.

## Status

Working

## Getting Started

```bash
skaffold dev
```

## Notes

- It seems like each secret is output to a single file in the volume attached,
  not to a single file.
