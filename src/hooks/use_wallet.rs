use dioxus::{document::eval, prelude::*};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::{GatewayError, GatewayResult};

use super::{use_miner_is_active, IsActive};

pub fn use_wallet() -> Signal<Wallet> {
    use_context::<Signal<Wallet>>()
}

pub fn use_wallet_provider() {
    let mut signal = use_context_provider(|| Signal::new(Wallet::Disconnected));
    let mut miner_is_active = use_miner_is_active();
    let mut eval = eval(
        r#"
            window.addEventListener("ore-pubkey", (event) => {
                dioxus.send(event.detail.pubkey);
            });
        "#,
    );
    spawn(async move {
        while let Ok(json_val) = eval.recv().await {
            let pubkey_result: Result<Pubkey, serde_json::Error> = serde_json::from_value(json_val);
            match pubkey_result {
                Ok(pubkey) => {
                    signal.set(Wallet::Connected(pubkey));
                }
                Err(_) => {
                    signal.set(Wallet::Disconnected);
                    miner_is_active.set(IsActive(false));
                }
            }
        }
    });
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
            Wallet::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
        }
    }
}
