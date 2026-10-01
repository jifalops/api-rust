# Build from the repository root:
#   docker build -t api-rust -f deploy/docker/Dockerfile .
FROM rust:slim-bookworm AS builder

# Compile the sqlx query macros against the committed lib/.sqlx cache instead of
# a live database, so the image builds with no Postgres available.
ENV SQLX_OFFLINE=true

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY lib ./lib

# Cache mounts keep the registry and compiled dependencies between builds. The
# target dir isn't part of the image, so the binary is copied out of it.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release && cp target/release/api-rust /usr/local/bin/

# Production
FROM gcr.io/distroless/cc

WORKDIR /app

COPY --from=builder /usr/local/bin/api-rust /app/

ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

CMD ["/app/api-rust"]
