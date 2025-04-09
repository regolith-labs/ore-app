#[cfg(feature = "android")]
mod updater_android;
#[cfg(all(not(feature = "web"), not(feature = "android")))]
mod updater_native;
#[cfg(feature = "web")]
mod updater_web;

#[cfg(feature = "android")]
pub use updater_android::*;
#[cfg(all(not(feature = "web"), not(feature = "android")))]
pub use updater_native::*;
#[cfg(feature = "web")]
pub use updater_web::*;
