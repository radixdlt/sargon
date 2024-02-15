#!/usr/bin/env zsh

set -e
set -u

# Credits: https://github.com/ianthetechie/uniffi-starter/blob/main/rust/build-ios.sh
# NOTE: You MUST run this every time you make changes to the core. Unfortunately, calling this from Xcode directly
# does not work so well.

# In release mode, we create a ZIP archive of the xcframework and update Package.swift with the computed checksum.
# This is only needed when cutting a new release, not for local development.
release=false

for arg in "$@"
do
    case $arg in
        --release)
            release=true
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
#
# * Only build one simulator arch for local development (we build both since many still use Intel Macs)
# * Option to do debug builds instead for local development
fat_simulator_lib_dir="target/ios-simulator-fat/release"

generate_ffi() {
  echo "📦 Generating framework module mapping and FFI bindings"
  cargo run --bin uniffi-bindgen generate --library target/aarch64-apple-ios/release/lib$1.dylib --language swift --out-dir target/uniffi-xcframework-staging
  mkdir -p apple/Sources/UniFFI/
  mv target/uniffi-xcframework-staging/*.swift apple/Sources/UniFFI/
  mv target/uniffi-xcframework-staging/$1FFI.modulemap target/uniffi-xcframework-staging/module.modulemap  # Convention requires this have a specific name
}

create_fat_simulator_lib() {
  echo "📦 Creating a fat library for x86_64 and aarch64 simulators"
  mkdir -p $fat_simulator_lib_dir
  lipo -create target/x86_64-apple-ios/release/lib$1.a target/aarch64-apple-ios-sim/release/lib$1.a -output $fat_simulator_lib_dir/lib$1.a
}

build_xcframework() {
  # Builds an XCFramework
  echo "📦 Generating XCFramework"
  rm -rf target/ios  # Delete the output folder so we can regenerate it
  OUTDIR="target/ios/lib$1-rs.xcframework"
  xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/lib$1.a -headers target/uniffi-xcframework-staging \
    -library target/ios-simulator-fat/release/lib$1.a -headers target/uniffi-xcframework-staging \
    -output $OUTDIR

  if $release; then
    echo "📦 ('release' is true) Building xcframework archive"
    zip -r target/ios/lib$1-rs.xcframework.zip target/ios/lib$1-rs.xcframework
    checksum=$(swift package compute-checksum target/ios/lib$1-rs.xcframework.zip)
    version=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="sargon") .version')
    sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1$version\2/g" Package.swift
    sed -i "" -E "s/(let releaseChecksum = \")[^\"]+(\")/\1$checksum\2/g" Package.swift
  else
    echo "📦 'release' is false"
  fi
  echo "$OUTDIR"
}


me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";

PARENT_DIRECTORY="${DIR%/../../*}"
echo "📦 Start of '$me' (see: '$DIR/$me')"
cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.

cargo build --lib --release --target x86_64-apple-ios
cargo build --lib --release --target aarch64-apple-ios-sim
cargo build --lib --release --target aarch64-apple-ios

basename=sargon
generate_ffi $basename
create_fat_simulator_lib $basename
ZIP_PATH=$(build_xcframework $basename)
echo "📦 ✅ End of '$me', xcframework can be found at"

# This print MUST be the last print.
# The path is read by `release.sh` script.
# This is probably terrible... but I'm a Rust/Swift dev, not a bash script dev...
echo "$ZIP_PATH"