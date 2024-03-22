#!/usr/bin/env zsh

set -e
set -u

# By default we test with code coverage and display details (lines missed)

export_code_cov=false
testonly=false # if true, no code coverage will happen
summary=false # if true, code coverage will only show summary, no details
code_cov_report_file_path=""

for arg in "$@"
do
    case $arg in
        --codecov)
            export_code_cov=true
            code_cov_report_file_path="$2"
            shift # Remove --codecov from processing
            ;;
        --testonly)
            testonly=true
            shift # Remove --testonly from processing
            ;;
        --summary)
            summary=true
            shift # Remove --summary from processing
            ;;
        *)
            shift # Ignore other argument from processing
            ;;
    esac
done

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
# sh ./scripts/ios/build-sargon.sh --maconly || exit $?
echo "✨ Sargon built"

echo "✨ Calling 'swift test'"
if $testonly; then
    swift test
else
    swift test --enable-code-coverage
    COV_BUILD_FOLDER="$(sh ./scripts/ios/get_cov_bin_path.sh)"
fi

COV_ARGS="$COV_BUILD_FOLDER -instr-profile=\"$SWIFT_CODE_COV_DATA_PATH\" -ignore-filename-regex=\".build|Tests|UniFFI/Sargon.swift\""
NON_EXPORT_COV_ARGS="$COV_ARGS -region-coverage-lt=99 -use-color"
if $summary; then
    eval "xcrun llvm-cov report $NON_EXPORT_COV_ARGS"
elif $export_code_cov; then
    eval "xcrun llvm-cov export -format="lcov" $COV_ARGS > $code_cov_report_file_path"
elif [[ "$testonly" = false ]]; then # details
    eval "xcrun llvm-cov show $NON_EXPORT_COV_ARGS"
fi