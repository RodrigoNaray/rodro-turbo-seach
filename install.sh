#!/bin/sh
set -eu
project_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
if [ -f "$project_dir/bin/rodro-turbo-search" ]; then
    binary="$project_dir/bin/rodro-turbo-search"
else
    if ! command -v cargo >/dev/null 2>&1; then
        if [ -f "$HOME/.cargo/env" ]; then . "$HOME/.cargo/env"; fi
    fi
    /usr/bin/python3 "$project_dir/scripts/build_release.py"
    binary="$project_dir/target/release/rodro-turbo-search"
fi
exec /usr/bin/python3 "$project_dir/scripts/setup.py" install --binary "$binary"
