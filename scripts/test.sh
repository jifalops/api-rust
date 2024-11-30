#!/usr/bin/env bash

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

"$script_dir/test-unit.sh"
exit_code=$?

"$script_dir/test-e2e.sh"
exit_code=$((exit_code + $?))

exit $exit_code
