#[cfg(not(feature = "web"))]
mod submit_transaction_native;
#[cfg(feature = "web")]
mod submit_transaction_web;
mod transaction_status;

#[cfg(not(feature = "web"))]
pub use submit_transaction_native::*;
#[cfg(feature = "web")]
pub use submit_transaction_web::*;
pub use transaction_status::*;

// TODO Create a toast component to display the transaction status
// TODO Route all transactions through toast system