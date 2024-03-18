#!/usr/bin/env zsh

set -e
set -u

# Credits: https://github.com/ianthetechie/uniffi-starter/blob/main/rust/build-ios.sh
# NOTE: You MUST run this every time you make changes to the core. Unfortunately, calling this from Xcode directly
# does not work so well.

# In release mode, we create a ZIP archive of the xcframework and update Package.swift with the computed checksum.
# This is only needed when cutting a new release, not for local development.
release=false
TAG_OF_RELEASE=""

for arg in "$@"
do
    case $arg in
        --release-tag)
            release=true
            TAG_OF_RELEASE="$2"
            shift # Remove --release from processing
            ;;
        *)
            shift # Ignore other argument from processing
            ;;
    esac
done


# Potential optimizations for the future:
# regularly check: https://github.com/ianthetechie/uniffi-starter/blob/main/rust/build-ios.sh
# for improvements!

generate_ffi() {
  echo "📦 Generating framework module mapping and FFI bindings"
  cargo run --bin uniffi-bindgen generate --library target/aarch64-apple-ios/release/lib$1.dylib --language swift --out-dir target/uniffi-xcframework-staging
  mkdir -p apple/Sources/UniFFI/
  mv target/uniffi-xcframework-staging/*.swift apple/Sources/UniFFI/
  mv target/uniffi-xcframework-staging/$1FFI.modulemap target/uniffi-xcframework-staging/module.modulemap  # Convention requires this have a specific name
}


build_xcframework() {
  # Builds an XCFramework
  echo "📦 Generating XCFramework"
  rm -rf target/swift  # Delete the output folder so we can regenerate it
  local XCFRAME_PATH="target/swift/lib$1-rs.xcframework"
  local XCFRAME_ZIP_PATH="$XCFRAME_PATH.zip"
  xcodebuild -create-xcframework \
    -library target/aarch64-apple-darwin/release/lib$1.a -headers target/uniffi-xcframework-staging \
    -library target/aarch64-apple-ios/release/lib$1.a -headers target/uniffi-xcframework-staging \
    -library target/aarch64-apple-ios-sim/release/lib$1.a -headers target/uniffi-xcframework-staging \
    -output $XCFRAME_PATH

  if $release; then
    local CHKSUM="RELEASE_WAS_FALSE_THUS_NO_CHECKSUM"
    echo "📦 ('release' is true) Building xcframework archive"
    zip -r $XCFRAME_ZIP_PATH $XCFRAME_PATH
    CHKSUM=$(swift package compute-checksum $XCFRAME_ZIP_PATH)
    sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1$TAG_OF_RELEASE\2/g" Package.swift
    sed -i "" -E "s/(let releaseChecksum = \")[^\"]+(\")/\1$CHKSUM\2/g" Package.swift
    echo "$CHKSUM;$XCFRAME_ZIP_PATH"
  else
    echo "BUILT_WITHOUT_RELEASE"
  fi
}


me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";

PARENT_DIRECTORY="${DIR%/../../*}"

if $release; then
  echo "📦 Start of '$me' (see: '$DIR/$me'), TAG_OF_RELEASE = '$TAG_OF_RELEASE'"
else
  echo "📦 Start of '$me' (see: '$DIR/$me')"
fi

cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.

cargo build --lib --release --target aarch64-apple-darwin
cargo build --lib --release --target aarch64-apple-ios-sim
cargo build --lib --release --target aarch64-apple-ios

basename=sargon
generate_ffi $basename
OUTPUT_OF_BUILD=$(build_xcframework $basename)
echo "📦 ✅ End of '$me', output"

# This print MUST be the last print.
# The path is read by `release.sh` script.
# This is probably terrible... but I'm a Rust/Swift dev, not a bash script dev...
echo "$OUTPUT_OF_BUILD"