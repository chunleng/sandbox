# Rig

Sandbox to test using `rig` crate, an LLM framework.

## Status

Working

## Getting Started

```bash
# Set up llama.cpp model
curl -L https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf --output ./model/qwen2.5-0.5b-instruct-q4.gguf

cargo run --example=agent-as-tool
```
