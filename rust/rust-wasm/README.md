# Sandbox

This sandbox project is used to learn/test out concepts in Web Assembly

## Status

Working

## Getting Started

Some tools that can be used with this project.

```bash
# In general, rust needs the wasm target to work with wasm
rustup target add wasm32-unknown-unknown

# For easy serving a html file
cargo install miniserve

# For building Web Assembly
cargo install wasm-pack

```

For each project, it can be run with the following command:

```bash
# With no bundler
env -C app/web wasm-pack build --target web
miniserve ./app/web --header "Cache-Control:no-cache" --index index.html -p 8080 # http://localhost:8080

# With web worker
env -C app/web_worker wasm-pack build --target no-modules
miniserve ./app/web_worker/ --header "Cache-Control:no-cache" --index index.html -p 8080 # http://localhost:8080
```
