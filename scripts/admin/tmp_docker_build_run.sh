#!/usr/bin/env bash

set -e

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

set -o allexport
source "$script_dir/../../.env"
set +o allexport

pushd "$script_dir/../../deploy/docker" > /dev/null
docker build -t api-rust -f Dockerfile ../..
popd > /dev/null

docker run -p "3010:3000" -e RUST_LOG=${PROD_RUST_LOG} -e POSTGRES_URI=${PROD_POSTGRES_URI} api-rust
