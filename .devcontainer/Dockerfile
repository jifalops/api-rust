FROM jifalops/rust-lambda:latest

# Install additional packages
RUN sudo apt-get update && sudo apt-get install -y --no-install-recommends \
  trash-cli

RUN rustup default 1.86 && \
  rustup component add rustfmt && \
  rustup component add clippy