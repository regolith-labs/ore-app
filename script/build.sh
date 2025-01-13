cargo install dioxus-cli --version 0.5.7 --locked
cargo install -f wasm-bindgen-cli --version 0.2.93 
rustup target add wasm32-unknown-unknown 
dx build --release 
cd serve 
cargo build --release
