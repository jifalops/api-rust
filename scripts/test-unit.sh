#!/usr/bin/env bash

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
pushd "$script_dir/.." > /dev/null

cargo test --doc --no-fail-fast
exit_code=$?
cargo nextest run --profile unit
exit_code=$((exit_code + $?))

popd > /dev/null

exit $exit_code
