mod use_wallet;
// #[cfg(not(feature = "web"))]
pub mod use_wallet_native;
#[cfg(feature = "web")]
mod use_wallet_web;

pub use use_wallet::*;
// #[cfg(not(feature = "web"))]
pub use use_wallet_native::*;
#[cfg(feature = "web")]
pub use use_wallet_web::*;
