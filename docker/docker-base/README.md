# Docker Base

Base template for creating docker. For docker, usually it will be tied to an
image in my public repository.

## Status

Working

## Getting Started

Change the `TARGET_REPO` in [`Makefile`](./Makefile) and we are ready to go!

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
