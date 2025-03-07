mod error;
pub mod kamino;
pub mod meteora;
pub mod ore;
pub mod pool;
mod priority_fee;
pub mod rpc;
#[cfg(not(feature = "web"))]
mod rpc_native;
#[cfg(feature = "web")]
mod rpc_web;
pub mod solana;
pub mod spl;
mod utils;

pub use error::*;
pub use priority_fee::*;
pub use rpc::*;
#[cfg(not(feature = "web"))]
pub use rpc_native::*;
#[cfg(feature = "web")]
pub use rpc_web::*;
pub use utils::*;

pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";

pub struct Gateway<R: Rpc> {
    pub rpc: R,
    pub http: reqwest::Client,
}

impl<R: Rpc> Gateway<R> {
    pub fn new(rpc_url: String) -> Gateway<R> {
        Gateway {
            rpc: R::new(rpc_url),
            http: reqwest::Client::new(),
        }
    }
}
