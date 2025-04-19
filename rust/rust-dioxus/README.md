# Rust Dioxus

A sandbox project to test out the dioxus package.

## Status

Working

## Getting Started

```bash
# This is used for building the app
cargo install dioxus-cli

# Use the setup guide on how to setup emulator for mobile
# https://dioxuslabs.com/learn/0.6/guides/mobile


# To activate tailwindcss
npx tailwindcss@3 -i ./input.css -o ./assets/tailwind.css --watch
dx serve --example layout --target web
```

## Note

- I can't get Tailwindcss v4 to work as of writing. Therefore v3 is used here.
