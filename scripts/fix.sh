#!/usr/bin/env bash

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
pushd "$script_dir/.." > /dev/null

cargo sort lib
cargo sort macros
cargo fmt
cargo fix --allow-dirty
cargo clippy --fix --allow-dirty

popd > /dev/null
