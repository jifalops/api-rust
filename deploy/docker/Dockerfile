# Build
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /usr/src/app

COPY ../.. .

RUN cargo build --release --workspace

# Production
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binaries from builder
COPY --from=builder /usr/src/app/target/release/* /app/

# Set environment variables
ENV RUST_LOG=debug
ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

# Update with your API binary name
CMD ["/app/api-rust"]