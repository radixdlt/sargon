#!/usr/bin/env zsh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";
cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.

echo "✨ Start of '$me' (see: '$DIR/$me')"
echo "✨ PWD: $PWD"

echo "✨ Ensure 'useLocalFramework' is set to 'true' in Package.swift"
sh ./scripts/ios/ensure-is-local.sh || exit $?

echo "✨ Building Sargon..."
sh ./scripts/ios/build-sargon.sh || exit $?
echo "✨ Sargon built"

echo "✨ Calling 'swift test'"
swift test

exit 0;