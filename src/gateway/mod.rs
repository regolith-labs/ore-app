mod ore;
mod error;
mod solana;
mod spl;
mod utils;

use solana_client_wasm::WasmClient;
pub use error::*;
pub use utils::*;

// pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";
pub const RPC_URL: &str = "https://rainy-alis-fast-mainnet.helius-rpc.com";

pub struct Gateway {
    pub rpc: WasmClient,
    pub http: reqwest::Client,
}

impl Gateway {
    pub fn new(rpc_url: String) -> Self {
        Gateway {
            rpc: WasmClient::new(&rpc_url),
            http: reqwest::Client::new(),
        }
    }
}
