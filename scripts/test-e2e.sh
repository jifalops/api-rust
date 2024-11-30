
#!/usr/bin/env bash

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
pushd "$script_dir/.." > /dev/null

cargo nextest run --profile e2e
exit_code=$?

popd > /dev/null

exit $exit_code
