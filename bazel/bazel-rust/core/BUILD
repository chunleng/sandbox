load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "core",
    crate_name = "app_core",
    srcs = ["src/lib.rs"],
    visibility = ["//:lintable", "//app:__pkg__"]
)

rust_test(
    name = "test",
    crate = ":core"
)
