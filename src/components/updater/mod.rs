#[cfg(not(feature = "web"))]
mod updater_native;
#[cfg(feature = "web")]
mod updater_web;

#[cfg(not(feature = "web"))]
pub use updater_native::*;
#[cfg(feature = "web")]
pub use updater_web::*;
