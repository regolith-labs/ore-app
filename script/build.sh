#!/bin/bash
cargo install dioxus-cli --version 0.6.0 --locked
cargo install wasm-bindgen-cli --version 0.2.99 
rustup target add wasm32-unknown-unknown
cargo vendor -p wasm-bindgen
dx build --release
