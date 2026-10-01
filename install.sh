#!/usr/bin/env bash
# Installs the tooling the justfile recipes expect. Docker is assumed to be
# installed already.
set -euo pipefail

cd "$(dirname "$0")"

if ! command -v rustup >/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
  source "$HOME/.cargo/env"
fi

# Installs the toolchain and components pinned in rust-toolchain.toml.
rustup toolchain install

# Prebuilt binaries where available, instead of compiling each tool from source.
if ! command -v cargo-binstall >/dev/null; then
  curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
fi
cargo binstall --no-confirm just cargo-watch cargo-nextest cargo-sort

# Built from source to get only the Postgres driver and rustls.
cargo install --locked sqlx-cli --no-default-features --features postgres,rustls

# Bundles Zig, which `just lambda` uses to cross-compile for arm64.
if ! command -v cargo-lambda >/dev/null; then
  curl -fsSL https://cargo-lambda.info/install.sh | sh
fi

if [ ! -f .env ]; then
  cp .env.example .env
  echo "Created .env from .env.example"
fi
