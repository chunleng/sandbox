# Rust Dioxus

A sandbox project to test out the dioxus package.

## Status

Working

## Getting Started

```bash
# In general, rust needs the wasm target to work with wasm
rustup target add wasm32-unknown-unknown

# For building Web Assembly
cargo install wasm-pack

# This is used for building the app
cargo install dioxus-cli

# Use the setup guide on how to setup emulator for mobile
# https://dioxuslabs.com/learn/0.6/guides/mobile

# To activate tailwindcss
npx tailwindcss@3 -i ./input.css -o ./assets/tailwind.css --watch

# example for layout
dx serve --example layout --target web

# example for state
dx serve --example state --target web

# example for webworker
dx serve --example web_worker --target web

# example for wasm sqlite
env -C ../rust-wasm/app/sqlite wasm-pack build --target web --release --out-dir ../../../rust-dioxus/static/sqlite/
dx serve --example wasm_sqlite --target web/desktop/ios/android

# example for use_action
# This is a temporary workaround for https://github.com/DioxusLabs/dioxus/pull/3617
dx serve --example use_action --target web
```

## Note

- I can't get Tailwindcss v4 to work as of writing. Therefore v3 is used here.
- For wasm_sqlite, a stable link is needed for JS to show the asset properly.
  Therefore in `Dioxus.toml`, `asset_dir = "static"` is added
