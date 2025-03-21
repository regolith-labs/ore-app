#[cfg(feature = "web")]
pub use web_time::*;

#[cfg(not(feature = "web"))]
pub use std::time::*;

#[cfg(feature = "web")]
pub async fn sleep(ms: u64) {
    gloo_timers::future::sleep(std::time::Duration::from_millis(ms)).await;
}

#[cfg(not(feature = "web"))]
pub async fn sleep(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}
