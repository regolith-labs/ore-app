#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Configuration ---
# Relative path to the root of the Dioxus-generated Android project
ANDROID_PROJECT_DIR="target/dx/ore-app/release/android"
# Relative path to the desired output directory for the AAB
OUTPUT_DIR="dist/android"
# The final name for the AAB file
FINAL_AAB_NAME="ore-app-release.aab"
# --- End Configuration ---

# Get the absolute path to the project root (where the script is located)
PROJECT_ROOT=$(pwd)

# Absolute paths based on the project root
ABS_ANDROID_PROJECT_DIR="$PROJECT_ROOT/$ANDROID_PROJECT_DIR"
ABS_OUTPUT_DIR="$PROJECT_ROOT/$OUTPUT_DIR"
ABS_FINAL_AAB_PATH="$ABS_OUTPUT_DIR/$FINAL_AAB_NAME"

# Check if the Android project directory exists
if [ ! -d "$ABS_ANDROID_PROJECT_DIR" ]; then
  echo "Error: Android project directory not found at $ABS_ANDROID_PROJECT_DIR"
  echo "Please ensure you have run 'dx build --platform android' at least once."
  exit 1
fi

echo "Navigating to Android project root: $ABS_ANDROID_PROJECT_DIR"
cd "$ABS_ANDROID_PROJECT_DIR"

# Navigate into the actual Gradle project directory
echo "Navigating into Gradle project: app"
cd "app"

# Ensure gradlew is executable (now relative to the 'app' directory)
GRADLEW_PATH="./gradlew"
if [ ! -x "$GRADLEW_PATH" ]; then
  echo "Making gradlew executable..."
  chmod +x "$GRADLEW_PATH"
fi

echo "Running Gradle bundleRelease task from $(pwd)..." # Should now be inside 'app'
# Run the clean and bundleRelease tasks.
echo "Running Gradle clean and bundleRelease tasks..."
if "$GRADLEW_PATH" clean bundleRelease; then
  echo "Gradle clean and bundleRelease successful."
else
  echo "Error: Gradle build failed."
  # Change back to project root before exiting on failure
  cd "$PROJECT_ROOT"
  exit 1
fi

# Define the expected location of the generated AAB (relative to the 'app' directory where gradle runs)
# The output is inside the 'app' module's build directory, which is nested
DEFAULT_AAB_PATH="app/build/outputs/bundle/release/app-release.aab"
EXPECTED_AAB_DIR="app/build/outputs/bundle/release" # Define the directory relative to 'app'

# List the contents of the expected output directory for debugging
ABS_EXPECTED_AAB_DIR="$(pwd)/$EXPECTED_AAB_DIR" # Absolute path for clarity in logs
echo "Checking contents of expected output directory: $ABS_EXPECTED_AAB_DIR"
# Create the directory path just in case Gradle didn't, though it should have
mkdir -p "$EXPECTED_AAB_DIR" # Create relative path
ls -l "$EXPECTED_AAB_DIR"    # List relative path

# Check if the AAB file was created (using the relative path from the current 'app' dir)
if [ ! -f "$DEFAULT_AAB_PATH" ]; then
  echo "Error: Expected AAB file not found at $(pwd)/$DEFAULT_AAB_PATH"
  # Change back to project root before exiting on failure
  cd "$PROJECT_ROOT"
  exit 1
fi

echo "AAB generated at: $(pwd)/$DEFAULT_AAB_PATH"

# Create the output directory if it doesn't exist
echo "Ensuring output directory exists: $ABS_OUTPUT_DIR"
mkdir -p "$ABS_OUTPUT_DIR"

# Copy the AAB to the final destination (using path relative to 'app' dir for source)
echo "Copying AAB from $(pwd)/$DEFAULT_AAB_PATH to $ABS_FINAL_AAB_PATH"
cp "$DEFAULT_AAB_PATH" "$ABS_FINAL_AAB_PATH" # Source path is relative to current dir

# Navigate back to the original directory (project root)
cd "$PROJECT_ROOT"

echo "-------------------------------------"
echo "Android AAB build complete!"
echo "Output available at: $ABS_FINAL_AAB_PATH"
echo "-------------------------------------"

exit 0
