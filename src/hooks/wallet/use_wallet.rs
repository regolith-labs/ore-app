use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::gateway::{GatewayError, GatewayResult};

pub fn use_wallet() -> Signal<Wallet> {
    use_context::<Signal<Wallet>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Wallet {
    Connected(Pubkey),
    Disconnected,
}

pub trait GetPubkey {
    fn get_pubkey(&self) -> GatewayResult<Pubkey>;
}

impl GetPubkey for Signal<Wallet> {
    fn get_pubkey(&self) -> GatewayResult<Pubkey> {
        match *self.read() {
            Wallet::Connected(pubkey) => Ok(pubkey),
            Wallet::Disconnected => Err(GatewayError::WalletDisconnected),
        }
    }
}
