FROM rust:1.83.0-bookworm AS builder_base
WORKDIR /builder
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src && touch src/lib.rs && \
    cargo fetch && \
    rm -rf src

FROM builder_base AS dev_dependencies
RUN cargo install cargo-watch --version 8.5.3 --locked

FROM builder_base AS dev
COPY --from=dev_dependencies /usr/local/cargo/bin/cargo-watch /usr/local/cargo/bin/cargo-watch
COPY src src
CMD cargo watch -- cargo run


FROM builder_base AS builder_prod
COPY src src
RUN cargo build --release

# Make link path so that we don't have to care about the Architecture of the library folder later
RUN ln -s /usr/lib/$(uname -m)-linux-gnu /usr/lib/linux-gnu

FROM gcr.io/distroless/base-debian12 AS prod
COPY --from=builder_prod /usr/lib/linux-gnu/libgcc_s.so.1 /usr/lib/
COPY --from=builder_prod /builder/target/release/skaffold-rust-dev-prod /runapp
CMD ["/runapp"]
