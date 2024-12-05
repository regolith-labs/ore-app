mod solana;
mod ore;
mod utils;
mod spl;

pub use utils::*;
use solana_client_wasm::WasmClient;

pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";

pub struct Gateway {
    pub rpc: WasmClient,
}

impl Gateway {
    pub fn new(rpc_url: String) -> Self {
        Gateway {
            rpc: WasmClient::new(&rpc_url),
        }
    }
}
