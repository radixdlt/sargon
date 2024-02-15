#!/bin/bash
# Inspired by: https://gist.github.com/jerrymarino/1f9eb6a06c423f9744ea297d80193a9b
# Author: Jerry Marino - @jerrymarino
# Edited by: Alexander Cyon @Sajjon
# 

me=$(basename "$0")
# dir of script 
DIR="$( cd "$( dirname "${(%):-%x}" )" && pwd )";
# parent dir of that dir
PARENT_DIRECTORY="${DIR%/../*}"

BUNDLE_IDENTIFIER=works.rdx.planbok.Planbok;
# export DEBUGGER_ENABLED=1|0

APP_SCHEME_NAME=PlanbokApp
APP_XCODEPROJP_PATH=examples/iOS/App/$APP_SCHEME_NAME.xcodeproj
TARGET_IOS_VERSION=17.2
TARGET_IPHONE_NAME="iPhone 15"
IPHONE_SIM_DEST="platform=iOS Simulator,name=$TARGET_IPHONE_NAME,OS=$TARGET_IOS_VERSION"

echo "📦 Start of '$me' (see: '$DIR/$me')"
echo "IPHONE_SIM_DEST: $IPHONE_SIM_DEST"
cd "$PARENT_DIRECTORY" # go to parent of parent, which is project root.

BUILD_ROOT=$(xcodebuild -project $APP_XCODEPROJP_PATH -scheme $APP_SCHEME_NAME -destination $IPHONE_SIM_DEST build -quiet -showBuildSettings CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO | grep -m 1 'BUILD_ROOT' | grep -oEi "\/.*")


echo "build root is: '$BUILD_ROOT'"
APP_PATH="$BUILD_ROOT/Debug-iphonesimulator/$APP_SCHEME_NAME.app"

export SED_START="iOS $TARGET_IOS_VERSION"
export SED_END="tvOS"
export IOS_SIM_UDID=$(xcrun simctl list devices | sed -n "/$SED_START/,/$SED_END/p" | grep "$TARGET_IPHONE_NAME (" | grep -E -o -i "([0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12})");\

xcrun simctl boot "$IOS_SIM_UDID";\
xcrun simctl launch booted "$BUNDLE_IDENTIFIER"

echo "Running sim for $APP_SCHEME_NAME - $BUNDLE_IDENTIFIER"

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

echo "Installing app at path $APP_PATH"
xcrun simctl install booted $APP_PATH

if [[ $DEBUGGER_ENABLED == "1" ]]; then
	LAUNCH_DEBUGGER_ENABLED_FLAG=--wait-for-debugger
	USE_CONSOLE_FLAG=""
else
	USE_CONSOLE_FLAG=--console
	LAUNCH_DEBUGGER_ENABLED_FLAG=""
fi

LOG_FILE=/tmp/run_ios_sim.log
echo "Starting Sim for $APP_PATH" > $LOG_FILE

# Launch the app program into the booted sim
# - Run in the background
`xcrun simctl launch $LAUNCH_DEBUGGER_ENABLED_FLAG $USE_CONSOLE_FLAG booted $BUNDLE_IDENTIFIER 2>&1 >> $LOG_FILE` &