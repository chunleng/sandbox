load("@rules_rust//rust:defs.bzl", "rust_clippy")

package_group(
    name = "lintable",
    packages = ["//"]
)

rust_clippy(
    name = "clippy",
    deps = ["//core", "//app"]
)
