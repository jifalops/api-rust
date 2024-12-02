#!/usr/bin/env bash

set -e

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

pushd "$script_dir/../.." > /dev/null
cargo lambda build --release --features lambda --arm64
popd > /dev/null

set -o allexport
source "$script_dir/../../.env"
set +o allexport

pushd "$script_dir/../../deploy/aws_sam" > /dev/null
sam deploy --debug --parameter-overrides RustLog="$PROD_RUST_LOG" PostgresUri="$PROD_POSTGRES_URI"
popd > /dev/null
