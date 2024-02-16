#!/usr/bin/env zsh

# Inspired by: https://gist.github.com/jerrymarino/1f9eb6a06c423f9744ea297d80193a9b
# Author: Jerry Marino - @jerrymarino
# Edited by: Alexander Cyon @Sajjon

set -e
set -u

me=$(basename "$0")
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";

BUNDLE_IDENTIFIER=works.rdx.planbok.Planbok;

APP_SCHEME_NAME=PlanbokApp
APP_XCODEPROJP_PATH=examples/iOS/App/$APP_SCHEME_NAME.xcodeproj
TARGET_IOS_VERSION=17.2
TARGET_IPHONE_NAME="iPhone 15 Pro Max"

echo "📱 Start of '$me' (see: '$DIR/$me') 🔮"
cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.

export SED_START="iOS $TARGET_IOS_VERSION"
export SED_END="tvOS"

export IOS_SIM_UDID=$(xcrun simctl list devices | sed -n "/$SED_START/,/$SED_END/p" | grep "$TARGET_IPHONE_NAME (" | grep -E -o -i "([0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12})");\

IPHONE_SIM_DEST="platform=iOS Simulator,id=$IOS_SIM_UDID"
# echo "📱 IPHONE_SIM_DEST '$IPHONE_SIM_DEST'"
XCODEBUILD_CMD_BASE="xcodebuild -project $APP_XCODEPROJP_PATH -scheme $APP_SCHEME_NAME"
BUILD_CMD="$XCODEBUILD_CMD_BASE build -destination \"$IPHONE_SIM_DEST\" -configuration Debug -quiet -showBuildSettings CODE_SIGN_IDENTITY=\"\" CODE_SIGNING_REQUIRED=NO | grep -m 1 'BUILD_ROOT' | grep -oEi \"\/.*\""
echo "📱🛠️  Building app: '$APP_SCHEME_NAME'..."
BUILD_ROOT=$(eval $BUILD_CMD)
APP_PATH="$BUILD_ROOT/Debug-iphonesimulator/$APP_SCHEME_NAME.app"
echo "📱🛠️  Built app '$APP_SCHEME_NAME' , it is here: '$APP_PATH' ✅"

echo "📱 Starting simulator '$IOS_SIM_UDID'..."

# Open the simulator
open -a 'Simulator' --args -CurrentDeviceUDID $IOS_SIM_UDID

# Wait until there is a device booted

function booted_sim_ct() {
	echo `xcrun simctl list | grep Booted | wc -l | sed -e 's/ //g'`
}

while [ `booted_sim_ct` -lt 1 ]
do
	sleep 1
done

echo "📱 Started simulator '$IOS_SIM_UDID' ✅"

echo "📱📦 Installing app '$APP_SCHEME_NAME'..."
xcrun simctl install booted $APP_PATH
echo "📱📦 Installed app '$APP_SCHEME_NAME' ✅"



echo "📱🚀 Launching app '$APP_SCHEME_NAME'..."
`xcrun simctl launch --console booted $BUNDLE_IDENTIFIER` &
echo "📱🚀 Launched app '$APP_SCHEME_NAME' ✅"

echo "📱 End of '$me' ✨"