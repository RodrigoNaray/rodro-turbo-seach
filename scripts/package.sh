#!/bin/sh
# Creates a prebuilt user-installable archive for the current Linux architecture.
set -eu
project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$project_dir"
if ! command -v cargo >/dev/null 2>&1; then . "$HOME/.cargo/env"; fi
/usr/bin/python3 "$project_dir/scripts/build_release.py"
bundle="dist/rodro-turbo-search-linux-$(uname -m)"
mkdir -p "$bundle/bin" "$bundle/scripts" "$bundle/assets"
cp target/release/rodro-turbo-search "$bundle/bin/"
cp scripts/setup.py "$bundle/scripts/"
cp assets/rodro-turbo-search-v2-rayo.svg "$bundle/assets/"
cp install.sh README.md DEPENDENCIES.md "$bundle/"
tar --owner=0 --group=0 --numeric-owner -czf "$bundle.tar.gz" -C dist "$(basename "$bundle")"
printf 'Paquete: %s/%s.tar.gz\n' "$project_dir" "$bundle"
