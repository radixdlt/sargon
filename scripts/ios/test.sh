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
sh ./scripts/ios/build-sargon.sh --maconly || exit $?
echo "✨ Sargon built"

echo "✨ Calling 'swift test'"
if $testonly; then
    swift test
elif $summary; then
    swift test --enable-code-coverage && scripts/ios/display_code_cov_summary.sh
elif $export_code_cov; then
    swift test --enable-code-coverage && scripts/ios/export_code_cov.sh $code_cov_report_file_path
else
    swift test --enable-code-coverage && scripts/ios/display_code_cov_details.sh
fi