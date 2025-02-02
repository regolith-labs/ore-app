#[cfg(feature = "web")]
pub use web_time::*;

#[cfg(not(feature = "web"))]
pub use std::time::*;
