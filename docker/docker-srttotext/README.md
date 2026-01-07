# Docker srttotext

Docker built for running <https://github.com/oshenc/srttotext>.

## Status

Working

## Getting Started

```bash
# Build and deploy to TARGET_REPO
make

# Run locally
make run
make cmd=sh run
```

## Note

- Seems like Colima has problem with multi-platform build by default. To
  resolve, use the following command:

  ```bash
  docker buildx create --use --name super-builder
  ```
