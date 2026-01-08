# Sandbox

Sandbox project for rust crate `strum`, a convenient tool for making String
Enum.

Having String Enum is useful when we want to ensure type safetiness when
parsing from string to Enum. If the Enum and parsing of String to Enum is a
separate process, we might miss out on adding the parsing logic when a new Enum
is added, such as the following:

```rust
match some_string {
  "a" => SomeEnum::A,
  "b" => SomeEnum::B,
  _ => panic!()
}
```

If we add `SomeEnum::C`, we can conveniently forget about adding to the above
string parser, thus causing unnecessary problems.

## Status

Working

## Getting Started

```
cargo run
```
