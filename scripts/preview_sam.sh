#!/usr/bin/env bash

set -e
script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Use cargo lambda to build.
pushd "$script_dir/.." > /dev/null
cargo lambda build --release --features lambda
popd > /dev/null

# Use SAM to start the API.
pushd "$script_dir/../deploy/aws_sam" > /dev/null
sam local start-api --debug --host 0.0.0.0 --port 3001 \
  --parameter-overrides Arch="$(uname -m)" RustLog="$RUST_LOG" PostgresUri="$POSTGRES_URI"
popd > /dev/null
