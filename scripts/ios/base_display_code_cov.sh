#!/bin/sh

set -e
set -u

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
	-ignore-filename-regex="Tests|UniFFI/Sargon.swift" \
	-region-coverage-lt=99 \
	-use-color