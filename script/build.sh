#!/bin/bash
cargo install dioxus-cli
cargo install -f wasm-bindgen-cli --version 0.2.93
rustup target add wasm32-unknown-unknown
dx build --release
