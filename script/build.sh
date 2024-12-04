# Add git hash to environment
export GIT_HASH=$(git rev-parse HEAD)
cargo install dioxus-cli &&
cargo install -f wasm-bindgen-cli --version 0.2.93 &&
rustup target add wasm32-unknown-unknown &&  
RUSTFLAGS="--cfg git_hash=\"$GIT_HASH\"" dx build --release &&
cd serve && 
cargo build --release
