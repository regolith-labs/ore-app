#!/bin/bash
cargo install dioxus-cli --version 0.6.0 --locked
cargo install wasm-bindgen-cli --version 0.2.99 
dx -V
wasm-bindgen -V
which dx
which wasm-bindgen
rustup target add wasm32-unknown-unknown
dx build --release --verbose
