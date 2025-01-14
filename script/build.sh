#!/bin/bash
cargo install dioxus-cli --version 0.6.1 --locked
rustup target add wasm32-unknown-unknown
dx build --release --verbose
cd serve 
cargo build --release
