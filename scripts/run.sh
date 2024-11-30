#!/usr/bin/env bash

set -e

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
pushd "$script_dir/../lib" > /dev/null

cargo watch -x run

popd > /dev/null
