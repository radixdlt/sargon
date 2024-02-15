#!/usr/bin/env zsh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";


ZIP_PATH=$(sh $DIR/build-sargon.sh --release | tail -n 1)

# This print MUST be the last print.
# The path is read by `release.yml` CD pipeline!
# This is probably terrible... but I'm a Rust/Swift dev, not a bash script dev...
echo "$ZIP_PATH"