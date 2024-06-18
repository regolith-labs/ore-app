# Build app for apple and intel silicon
cargo build --features "desktop bundle" --target aarch64-apple-darwin --release
cargo build --features "desktop bundle" --target x86_64-apple-darwin --release

# Link builds into unified binary
rm target/release/ore-app
lipo -create \
  target/x86_64-apple-darwin/release/ore-app \
  target/aarch64-apple-darwin/release/ore-app \
  -output target/release/ore-app

# Bundle
cargo bundle --release --features "desktop bundle" --format osx

# Sign
codesign --force --deep \
  --sign "Developer ID Application: Nicholas Garfield (RP8738PY76)" \
  --entitlements AppEntitlements.entitlements \
  --timestamp \
  --options runtime target/release/bundle/osx/Ore.app

# Zip
cd target/release/bundle/osx
zip -r Ore.zip Ore.app

# Send to apple for notarization
xcrun notarytool submit Ore.zip --keychain-profile "OreAppNotarization" --wait

# Stable notarization to app
xcrun stapler staple Ore.app

# Rezip into new app
zip -r Ore-MacOS.zip Ore.app

# Open current directory in Finder
open .

# Move back to the top-level directory
cd ../../../../

