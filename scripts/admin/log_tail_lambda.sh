#!/usr/bin/env bash

set -e

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

set -o allexport
source "$script_dir/../../.env"
set +o allexport

aws logs tail "/aws/lambda/$PROD_LAMBDA_LOG_GROUP" --follow
