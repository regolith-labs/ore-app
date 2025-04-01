use std::fmt::Display;

use solana_sdk::signature::Signature;

#[derive(PartialEq, Clone, Debug)]
pub enum TransactionStatus {
    Waiting,
    Denied,
    Error,
    ErrorWithMessage(String),
    Timeout,
    Sending(u8),
    Done(Signature),
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::ErrorWithMessage(msg) => write!(f, "Error: {}", msg),
            _ => write!(f, "{:?}", self),
        }
    }
}
