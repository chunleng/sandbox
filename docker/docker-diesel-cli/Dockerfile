FROM rust:1.93.1 AS builder

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install diesel_cli@2.2.10 --locked

FROM ubuntu:26.04 AS runner

RUN apt-get update && \
    apt-get install -y libmariadb-dev libsqlite3-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diesel /bin/diesel

ENV DATABASE_URL=

WORKDIR /app
RUN echo 'diesel --database-url="${DATABASE_URL}" $@' > /entrypoint.sh \
    && chmod +x /entrypoint.sh

ENTRYPOINT [ "sh", "/entrypoint.sh"  ]
CMD [ "migration", "run" ]

