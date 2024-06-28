use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::{
    gateway::{ore_token_account_address, ore_token_account_address_v1, GatewayResult},
    wallet_adapter::WalletAdapter,
};

use super::use_gateway;

pub fn use_wallet_adapter() -> Signal<Option<WalletAdapter>> {
    use_context::<Signal<Option<WalletAdapter>>>()
}

pub fn use_wallet_adapter_provider() {
    let mut wallet_adapter = use_context_provider(|| Signal::new(None));
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
}

pub fn use_ore_balance_v1() -> Resource<Option<UiTokenAmount>> {
    let gateway = use_gateway();
    let wallet_adapter_signal = use_wallet_adapter();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            let maybe_wallet_adapter = *wallet_adapter_signal.read();
            match maybe_wallet_adapter {
                Some(wa) => {
                    let token_account_address = ore_token_account_address_v1(wa.pubkey);
                    gateway
                        .rpc
                        .get_token_account_balance(&token_account_address)
                        .await
                        .ok()
                }
                None => None,
            }
        }
    })
}
