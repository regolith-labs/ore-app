cargo install dioxus-cli &&
rustup target add wasm32-unknown-unknown &&  
dx build --release &&
cd serve && 
cargo build --release
