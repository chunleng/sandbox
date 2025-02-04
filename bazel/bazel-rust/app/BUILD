load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "app",
    srcs = ["src/main.rs"],
    deps = ["//core:core"],
    visibility = ["//:lintable"]
)

