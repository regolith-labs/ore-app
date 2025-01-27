use std::fmt::Display;

use solana_sdk::signature::Signature;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TransactionStatus {
    Waiting,
    Denied,
    Error,
    Timeout,
    Sending(u8),
    Done(Signature),
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
