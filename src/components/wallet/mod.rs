// #[cfg(not(feature = "web"))]
mod wallet_adapter_native;
#[cfg(feature = "web")]
mod wallet_adapter_web;
#[cfg(not(feature = "web"))]
mod wallet_drawer_native;
#[cfg(feature = "web")]
mod wallet_drawer_web;

#[cfg(not(feature = "web"))]
pub use wallet_adapter_native::*;
#[cfg(feature = "web")]
pub use wallet_adapter_web::*;
#[cfg(not(feature = "web"))]
pub use wallet_drawer_native::WalletDrawer;
#[cfg(feature = "web")]
pub use wallet_drawer_web::WalletDrawer;
