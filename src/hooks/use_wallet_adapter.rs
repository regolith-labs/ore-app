use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::wallet_adapter::WalletAdapter;

pub fn use_wallet_adapter() -> Signal<Option<WalletAdapter>> {
    let mut wallet_adapter = use_signal(|| None);
    let mut eval = eval(
        r#"
        console.log("eval this");
        window.addEventListener("ore-pubkey", (event) => {
            console.log(event.detail);
            dioxus.send(event.detail.pubkey.toBuffer().toJSON().data);
        });
        "#,
    );
    spawn(async move {
        dioxus_logger::tracing::info!("starting");
        while let Ok(json_val) = eval.recv().await {
            dioxus_logger::tracing::info!("json val: {}", json_val);
            let pubkey_result: Result<Pubkey, serde_json::Error> = serde_json::from_value(json_val);
            match pubkey_result {
                Ok(pubkey) => {
                    dioxus_logger::tracing::info!("pubkey: {}", pubkey);
                    wallet_adapter.set(Some(WalletAdapter { pubkey }));
                }
                Err(err) => {
                    dioxus_logger::tracing::info!("err: {}", err);
                    wallet_adapter.set(None);
                }
            };
        }
        log::info!("exited");
    });
    wallet_adapter
}
