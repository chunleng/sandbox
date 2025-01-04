# Sandbox

This is a repository where I experiment with code. Might contain incomplete
project, or maybe I might promote it to become my snippet or template in cookie
cutter. Who knows?

## Philosophy

A lot of codes I have written to test things are never preserved, and I will
think to myself, what did I do back then? This repository is meant to capture
every single things that I have tried, succeed or not

## Layout

The top layer of the folder contains folders of the following category:

- `template`: useful setups to start a project.
- `docker`: Docker image that I deploy to docker hub to mainly use for testing.
- `terraform`, `rust`...: Refers to the experiments I did in that language.
- `tool`: some quick tools I built mainly to use for testing.

Example:

```text
.
├─ docker
│  └─ docker-nginx-sample -> a sample nginx webpage
├─ rust
│  └─ tracing -> to test out Rust `tracing` library
├─ template
│  └─ rabbitmq -> commonly-used setup for rabbitmq
└─ tool
   └─ send-email -> tool for sending email
```

## Project README

In order not to be confused with things that are working and things that are
not, each project has a `README.md` and after the description, A status section
to show whether I have failed parts in the project.
