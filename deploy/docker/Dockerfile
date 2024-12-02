# Build
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

COPY ../../Cargo.toml ../../Cargo.lock ./
COPY ../../lib ./lib
COPY ../../macros ./macros

RUN cargo build --release --workspace

# Production
FROM gcr.io/distroless/cc

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