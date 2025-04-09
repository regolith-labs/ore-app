#[cfg(feature = "desktop")]
mod token_list;
#[cfg(feature = "android")]
mod wallet_adapter_android;
#[cfg(feature = "desktop")]
mod wallet_adapter_native;
#[cfg(feature = "web")]
mod wallet_adapter_web;
#[cfg(feature = "desktop")]
mod wallet_drawer_native;
#[cfg(feature = "web")]
mod wallet_drawer_web;

#[cfg(feature = "desktop")]
pub use token_list::*;
#[cfg(feature = "android")]
pub use wallet_adapter_android::*; // Added export for android adapter
#[cfg(feature = "desktop")]
pub use wallet_adapter_native::*;
#[cfg(feature = "web")]
pub use wallet_adapter_web::*;
#[cfg(feature = "desktop")] // Changed condition
pub use wallet_drawer_native::WalletDrawer;
#[cfg(feature = "web")]
pub use wallet_drawer_web::WalletDrawer;
// TODO: Add WalletDrawer for Android if needed
