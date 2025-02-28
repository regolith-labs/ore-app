#[cfg(not(feature = "web"))]
pub mod landing_native;

#[cfg(feature = "web")]
pub mod landing_web;

#[cfg(feature = "web")]
pub use landing_web::*;

#[cfg(not(feature = "web"))]
pub use landing_native::*;
