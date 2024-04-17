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

`git fetch --prune --tags`
function last_tag() {
    local out=`git tag --sort=committerdate | tail -1`
    echo $out
}
LAST_TAG=$(last_tag)
echo "🚢 🏷️  Last tag: $LAST_TAG"

# one liner from: https://stackoverflow.com/questions/8653126/8653732#comment65908962_8653732
NEXT_TAG=$(echo $(last_tag) | awk -F. -v OFS=. 'NF==1{print ++$NF}; NF>1{$NF=sprintf("%0*d", length($NF), ($NF+1)); print}')

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

echo "🚢 ensuring Swift Sargon build for release"
echo "🚢 Switch 'useLocalFramework' to 'true' in Package.swift"
# make script stateless
sed -i '' 's/let useLocalFramework = false/let useLocalFramework = true/' Package.swift

swift build -c release || exit $?
echo "🚢 Swift Sargon builds for release ✅"

echo "🚢 Switch 'useLocalFramework' to 'false' in Package.swift for release"
sed -i '' 's/let useLocalFramework = true/let useLocalFramework = false/' Package.swift

# We have .gitigored Sargon.swift because we dont need it in git history, but we
# need it for this release, so we must FORCE add it (since it is ignored).
GIT_ADD_CMD="git add --force Package.swift apple/Sources/UniFFI/Sargon.swift"
echo "🚢  Staging (potentially) changed files with: $GIT_ADD_CMD"
eval $GIT_ADD_CMD

GIT_COMMIT_CMD="git commit -m \"Release of '$NEXT_TAG' (updated Package.swift with new checksum and path to zip on Github, and maybe apple/Sources/UniFFI/Sargon.swift). This commit is not merged into main/develop branch (and need not be).\" --no-verify"
echo "🚢  Git commiting changes to Package.swift (and maybe Sargon.swift)"
eval $GIT_COMMIT_CMD

`git tag $NEXT_TAG`
echo "🚢 🏷️ 📡 Pushing tag: $(NEXT_TAG), but only tag, not commit."
`git push origin $NEXT_TAG`

# This MUST match whatever you we have declared in `$PROJECT_ROOT/Package.swift`
SWIFT_SARGON_BINARY_ASSET_NAME="libsargon-rs.xcframework.zip" 

GH_RELEASE_TITLE="v$NEXT_TAG"
RELEASE_CMD="gh release create $NEXT_TAG '$XCFRAME_ZIP_PATH#$SWIFT_SARGON_BINARY_ASSET_NAME' --generate-notes --notes-start-tag $LAST_TAG --title '$GH_RELEASE_TITLE'"
eval $RELEASE_CMD

echo "🚢  End of release script ✅"