#!/usr/bin/env zsh

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";
cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.

echo "🚢 Start of '$me' (see: '$DIR/$me')"
echo "🚢 PWD: $PWD"

echo "🚢 Ensure 'useLocalFramework' is set to 'false' in Package.swift"
sed -i '' 's/let useLocalFramework = true/let useLocalFramework = false/' Package.swift


`git fetch --prune --tags`
function last_tag() {
    local out=`git tag --sort=taggerdate | tail -1`
    echo $out
}
echo "🚢 🏷️ Last tag: $(last_tag)"

# one liner from: https://stackoverflow.com/a/8653732
NEXT_TAG=$(echo $(last_tag) | awk -F. -v OFS=. 'NF==1{print ++$NF}; NF>1{if(length($NF+1)>length($NF))$(NF-1)++; $NF=sprintf("%0*d", length($NF), ($NF+1)%(10^length($NF))); print}')

# output is: "<CHKSUM>;<$XCFRAME_ZIP_PATH>"
OUTPUT_OF_BUILD=`sh $DIR/build-sargon.sh --release-tag $NEXT_TAG | tail -n 1` || exit $?
if [[ "$OUTPUT_OF_BUILD" == "BUILT_WITHOUT_RELEASE" ]]; then
    echo "Error, failed to build, did you forget to pass '--release' to build script? Otherwise check if build-sargon.sh has recently been changed (to something incorrect...)"
    exit 1;
fi
CHECKSUM=`echo "$OUTPUT_OF_BUILD" | cut -d ";" -f 1` || exit $?
XCFRAME_ZIP_PATH=`echo "$OUTPUT_OF_BUILD" | cut -d ";" -f 2` || exit $?

echo "🚢  CHECKSUM: $CHECKSUM"
echo "🚢  XCFRAME_ZIP_PATH: $XCFRAME_ZIP_PATH"

exit 1 # TEMPORARY for testing

`git tag $NEXT_TAG`
echo "🚢 🏷️ 📡 Pushing tag: $(last_tag)"
`git push origin $NEXT_TAG`

# This MUST match whatever you we have declared in `$PROJECT_ROOT/Package.swift`
SWIFT_SARGON_BINARY_ASSET_NAME="libsargon-rs.xcframework.zip" 

GH_RELEASE_TITLE="Sargon Swift Only v$NEXT_TAG"
RELEASE_CMD="gh release create $NEXT_TAG '$XCFRAME_ZIP_PATH#$SWIFT_SARGON_BINARY_ASSET_NAME' --generate-notes --title '$GH_RELEASE_TITLE'"
eval $RELEASE_CMD

echo "🚢  End of release script ✅"