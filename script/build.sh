#!/bin/bash
# cargo install dioxus-cli --version 0.6.0
cargo install -f wasm-bindgen-cli --version 0.2.99
rustup target add wasm32-unknown-unknown
dx build --release
