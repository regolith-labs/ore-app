use std::fmt::Display;

use solana_sdk::signature::Signature;

#[cfg(not(feature = "web"))]
pub use super::submit_transaction_native::{invoke_signature, SubmitTransaction};
#[cfg(feature = "web")]
pub use super::submit_transaction_web::{invoke_signature, SubmitTransaction};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum InvokeSignatureStatus {
    Start,
    Waiting,
    DoneWithError,
    Timeout,
    Done(Signature),
}

impl Display for InvokeSignatureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
