#!/bin/sh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";

PARENT_DIRECTORY="${DIR%/../../*}"

cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.

echo "✨ Start of '$me' (see: '$DIR/$me')"
echo "✨ PWD: $PWD"

BIN_PATH="$(swift build --show-bin-path)"
XCTEST_PATH="$(find ${BIN_PATH} -name '*.xctest')"
COV_BIN=$XCTEST_PATH
if [[ "$OSTYPE" == "darwin"* ]]; then
    f="$(basename $XCTEST_PATH .xctest)"
    COV_BIN="${COV_BIN}/Contents/MacOS/$f"
fi

xcrun llvm-cov $1 \
	"${COV_BIN}" \
	-instr-profile=.build/debug/codecov/default.profdata \
	-ignore-filename-regex=".build|Tests|UniFFI/Sargon.swift"