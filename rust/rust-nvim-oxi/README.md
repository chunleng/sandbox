# Nvim-oxi

This is a sandbox project to test nvim-oxi crate, which allows the creation of
nvim plugin using rust. This crate does not use the neovim's RPC channel and
instead directly FFI into Vim's C code. Therefore, it's performant.

## Status

Working

## Getting Started

```bash
# Build the plugin
cargo build

# Try out on Nvim
nvim -u nvim/config.lua
```

## Note

- The `.cargo/config` setup is needed as `nvim-oxi` links will only be available at
  runtime
- When putting the library in the `runtimepath` folder, take note that the
  rust function name must match the library name.
