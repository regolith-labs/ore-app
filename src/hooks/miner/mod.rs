mod use_miner;
#[cfg(not(feature = "web"))]
mod use_miner_native;
#[cfg(feature = "web")]
mod use_miner_web;

pub use use_miner::*;
#[cfg(not(feature = "web"))]
pub use use_miner_native::*;
#[cfg(feature = "web")]
pub use use_miner_web::*;
