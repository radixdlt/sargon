#!/usr/bin/env zsh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";


ZIP_PATH=$(sh $DIR/build-sargon.sh --release)
echo "Zip path output of 'build-sargon.sh' was: '$ZIP_PATH'"
echo "END OF release.sh - bye bye."