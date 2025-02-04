# Bazel Rust

This is a sandbox to test bazel for rust.

## Status

Working

## Getting Started

```bash
bazel run @rules_rust//:rustfmt

# This will build everything
bazel build //...

# This will trigger test for just core
bazel test //core
```

## Notes

- There are 2 ways to link visibility, one is by using a `package_group`
  (referred by `//:package_group_name`) and another is to refer to the package
  directly (`//path:__pkg__`)
