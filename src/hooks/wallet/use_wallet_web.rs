use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::hooks::{use_miner_status, MinerStatus, Wallet};

pub fn use_wallet_provider() {
    let mut miner_status = use_miner_status();
    let mut signal = use_context_provider(|| Signal::new(Wallet::Disconnected));
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
                    miner_status.set(MinerStatus::Stopped);
                }
            }
        }
    });
}
