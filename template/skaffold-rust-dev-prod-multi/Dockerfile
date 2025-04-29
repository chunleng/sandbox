FROM rust:1.83.0-bookworm AS builder_base
WORKDIR /builder
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY app/backend/Cargo.toml app/backend/Cargo.toml
COPY app/frontend/Cargo.toml app/frontend/Cargo.toml
RUN mkdir -p app/backend/src && touch app/backend/src/lib.rs && \
    mkdir -p app/frontend/src && touch app/frontend/src/lib.rs && \
    cargo fetch && \
    rm app/backend/src/lib.rs && \
    rm app/frontend/src/lib.rs

FROM rust:1.83.0-bookworm AS dev_dependencies
RUN cargo install cargo-watch --version 8.5.3 --locked

FROM builder_base AS dev
COPY --from=dev_dependencies /usr/local/cargo/bin/cargo-watch /usr/local/cargo/bin/cargo-watch
COPY app app

FROM builder_base AS builder_prod_base
COPY app app
# Make link path so that we don't have to care about the Architecture of the library folder later
RUN ln -s /usr/lib/$(uname -m)-linux-gnu /usr/lib/linux-gnu

FROM builder_prod_base AS builder_prod_backend
RUN cargo build -p backend --release

FROM gcr.io/distroless/base-debian12 AS prod_backend
COPY --from=builder_prod_backend /usr/lib/linux-gnu/libgcc_s.so.1 /usr/lib/
COPY --from=builder_prod_backend /builder/target/release/backend /runapp
CMD ["/runapp"]

FROM builder_prod_base AS builder_prod_frontend
RUN cargo build -p frontend --release

FROM gcr.io/distroless/base-debian12 AS prod_frontend
COPY --from=builder_prod_frontend /usr/lib/linux-gnu/libgcc_s.so.1 /usr/lib/
COPY --from=builder_prod_frontend /builder/target/release/frontend /runapp
CMD ["/runapp"]
