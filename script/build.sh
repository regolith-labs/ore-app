#!/bin/bash
cargo install dioxus-cli --version 0.6.1 --locked
rustup target add wasm32-unknown-unknown
dx build --release

# Copy static assets to web/public
# cp public/icon.png target/dx/ore-app/release/web/public/assets
# cp public/metadata.json target/dx/ore-app/release/web/public/assets
# cp public/favicon.png target/dx/ore-app/release/web/public/assets
# cp -R public/fonts target/dx/ore-app/release/web/public/assets
# ls -la target/dx/ore-app/release/web/public/assets