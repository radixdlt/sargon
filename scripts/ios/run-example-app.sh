#!/bin/bash
# Inspired by: https://gist.github.com/jerrymarino/1f9eb6a06c423f9744ea297d80193a9b
# Author: Jerry Marino - @jerrymarino
# Edited by: Alexander Cyon @Sajjon
# 

me=$(basename "$0")
# dir of script 
REL_DIR=$0:P
DIR="$( cd "$( dirname "$REL_DIR" )" && pwd )";
BUNDLE_IDENTIFIER=works.rdx.planbok.Planbok;

APP_SCHEME_NAME=PlanbokApp
APP_XCODEPROJP_PATH=examples/iOS/App/$APP_SCHEME_NAME.xcodeproj
TARGET_IOS_VERSION=17.2
TARGET_IPHONE_NAME="iPhone 15 Pro Max"

echo "✨📱 Start of '$me' (see: '$DIR/$me')"
cd "$DIR" 
cd "../../" # go to parent of parent, which is project root.
echo "✨ pwd: $PWD"

export SED_START="iOS $TARGET_IOS_VERSION"
export SED_END="tvOS"

export IOS_SIM_UDID=$(xcrun simctl list devices | sed -n "/$SED_START/,/$SED_END/p" | grep "$TARGET_IPHONE_NAME (" | grep -E -o -i "([0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12})");\

IPHONE_SIM_DEST="platform=iOS Simulator,id=$IOS_SIM_UDID"
echo "✨📱 IPHONE_SIM_DEST '$IPHONE_SIM_DEST'"
XCODEBUILD_CMD_BASE="xcodebuild build -project $APP_XCODEPROJP_PATH -scheme $APP_SCHEME_NAME"
BUILD_CMD="$XCODEBUILD_CMD_BASE -destination \"$IPHONE_SIM_DEST\" -configuration Debug -quiet CODE_SIGN_IDENTITY=\"\" CODE_SIGNING_REQUIRED=NO"
eval $BUILD_CMD
PRINT_API_DIR_CMD="$XCODEBUILD_CMD_BASE -showBuildSettings"
BUILD_ROOT=$($PRINT_API_DIR_CMD | grep -m 1 'BUILD_ROOT' | grep -oEi "\/.*")

APP_PATH="$BUILD_ROOT/Debug-iphonesimulator/$APP_SCHEME_NAME.app"

echo "APP_PATH: $APP_PATH"

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
# - Pipe the output to a log file
# - Run in the background
`xcrun simctl launch $LAUNCH_DEBUGGER_ENABLED_FLAG $USE_CONSOLE_FLAG booted $BUNDLE_IDENTIFIER 2>&1 >> $LOG_FILE` &


echo "✨📱 End of '$me' (see: '$DIR/$me')"