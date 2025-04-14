#!/bin/bash
# android.update.sh - Build an APK set from the AAB and install it on a connected Android device

# Paths
BUNDLETOOL_JAR="$HOME/bundletool-all-1.18.1.jar" # Make sure this path is correct for your system
AAB_FILE="./dist/android/ore-app-release.aab"    # Updated path based on android.bundle.sh output
OUTPUT_APKS="./dist/android/OreApp.apks"         # Output APKS to the same directory

# Check if ADB is installed
if ! command -v adb &>/dev/null; then
  echo "ADB is not installed. Please install it (e.g., using Homebrew: brew install android-platform-tools) and try again."
  exit 1
fi

# Ensure a device is connected
DEVICE_COUNT=$(adb devices | sed '1d' | grep -w "device" | wc -l)
if [ "$DEVICE_COUNT" -eq 0 ]; then
  echo "No device connected. Connect your Android device with USB debugging enabled and try again."
  exit 1
fi

echo "Building APK set from AAB..."

# Ensure the output directory exists
OUTPUT_DIR=$(dirname "$OUTPUT_APKS")
echo "Ensuring output directory exists: $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Remove existing APK set if it exists
if [ -f "$OUTPUT_APKS" ]; then
  echo "Output file $OUTPUT_APKS already exists, removing it..."
  rm "$OUTPUT_APKS"
fi

# Build a universal APK set using bundletool
java -jar "$BUNDLETOOL_JAR" build-apks \
  --bundle="$AAB_FILE" \
  --output="$OUTPUT_APKS" \
  --mode=universal

if [ $? -ne 0 ]; then
  echo "Error: Failed to build the APK set."
  exit 1
fi

echo "APK set built successfully at $OUTPUT_APKS"

echo "Installing APK set on connected device..."
# Install the APK set on the connected device
java -jar "$BUNDLETOOL_JAR" install-apks --apks="$OUTPUT_APKS"

if [ $? -ne 0 ]; then
  echo "Error: Failed to install the APK set on the device."
  exit 1
fi

echo "Installation complete! Your app is now updated on your device."
