# Rust Tokio

A sandbox project to test out the rust library `tokio`.

## Status

Working

## Getting Started

```bash
cargo run --example mutex
cargo run --example watchfile

# To test `watchfile`
echo -n "a" > test_a
echo -n "b" > test_b
# Switch between test_a and test_b to test
ln -sf test_a test
ln -sf test_b test
```

## Notes

- For `watchfile` example, I have created it to resolve the problem with
  `notify` crate. Because symlink doesn't get tracked directly for `notify`
  (`notify` tracks the file it points to), it is hard to be used for watching
  secret or config map file changes in Kubernetes cluster.
