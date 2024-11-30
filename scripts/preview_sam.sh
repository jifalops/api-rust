#!/usr/bin/env bash

set -e

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
pushd "$script_dir/../deploy/aws_sam" > /dev/null

arch=$(uname -m)
export SAM_BUILD_MODE=debug
sam build --debug --parameter-overrides Arch="$arch"
sam local start-api --parameter-overrides Arch="$arch" --port 3001

popd > /dev/null
