#!/bin/bash
# Build the Android shared library via cargo-ndk and generate Uniffi Kotlin bindings

# Exit immediately if a command exits with a non-zero status
set -e

# Step 1: Build the shared library for Android using cargo-ndk
echo "Building Android shared library for arm64-v8a using cargo-ndk..."
cargo ndk -t arm64-v8a -- build --release
echo "Shared library built successfully."

# Step 2: Generate Uniffi Kotlin bindings
LIBRARY_PATH="target/aarch64-linux-android/release/libwallet_adapter_android.so"
OUTPUT_DIR="out"

echo "Generating Uniffi Kotlin bindings..."
cargo run --bin uniffi-bindgen generate \
  --library "$LIBRARY_PATH" \
  --language kotlin \
  --out-dir "$OUTPUT_DIR"

echo "Uniffi Kotlin bindings generated in the '$OUTPUT_DIR' directory."
