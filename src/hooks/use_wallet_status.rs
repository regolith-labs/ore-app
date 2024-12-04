use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

pub fn use_wallet_status() -> Signal<WalletStatus> {
    use_context::<Signal<WalletStatus>>()
}

pub fn use_wallet_status_provider() {
    let mut signal = use_context_provider(|| Signal::new(WalletStatus::Disconnected));
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
                Ok(pubkey) => signal.set(WalletStatus::Connected(pubkey)),
                Err(_) => signal.set(WalletStatus::Disconnected),
            }
        }
    });
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WalletStatus {
    Connected(Pubkey),
    Disconnected,
}
