use base64::Engine;
use dioxus::prelude::*;
use ore_api::consts::{TOKEN_DECIMALS, TOKEN_DECIMALS_V1};
use solana_client_wasm::solana_sdk::{
    pubkey::Pubkey, signature::Signature, transaction::Transaction,
};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{ore_token_account_address, ore_token_account_address_v1};
use crate::hooks::UiTokenAmountDefault;

use super::use_gateway;

pub fn use_wallet_adapter() -> Signal<WalletAdapter> {
    use_context::<Signal<WalletAdapter>>()
}

pub fn use_wallet_adapter_provider() {
    let mut signal = use_context_provider(|| Signal::new(WalletAdapter::Disconnected));
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
                Ok(pubkey) => signal.set(WalletAdapter::Connected(pubkey)),
                Err(_) => signal.set(WalletAdapter::Disconnected),
            }
        }
    });
}

// we only have one resource per hook
// until we update to latest dioxus from git
// so will have to pack many future here
// in a big result if we want more reactive async data
pub fn use_ore_balances() -> Resource<Option<Balances>> {
    let gateway = use_gateway();
    let signal = use_wallet_adapter();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            match *signal.read() {
                WalletAdapter::Connected(pubkey) => {
                    let token_account_address_v1 = ore_token_account_address_v1(pubkey);
                    let token_account_address_v2 = ore_token_account_address(pubkey);
                    let balance_v1 = gateway
                        .rpc
                        .get_token_account_balance(&token_account_address_v1)
                        .await
                        .unwrap_or(UiTokenAmount::default(TOKEN_DECIMALS_V1));
                    let balance_v2 = gateway
                        .rpc
                        .get_token_account_balance(&token_account_address_v2)
                        .await
                        .unwrap_or(UiTokenAmount::default(TOKEN_DECIMALS));
                    Some(Balances {
                        v1: balance_v1,
                        _v2: balance_v2,
                    })
                }
                WalletAdapter::Disconnected => None,
            }
        }
    })
}

pub fn invoke_signature(tx: Transaction, mut signal: Signal<InvokeSignatureStatus>) {
    signal.set(InvokeSignatureStatus::Waiting);
    let mut eval = eval(
        r#"
        let msg = await dioxus.recv();
        let signed = await window.OreTxSigner({b64: msg});
        dioxus.send(signed);
        "#,
    );
    match bincode::serialize(&tx) {
        Ok(vec) => {
            let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
            let res = eval.send(serde_json::Value::String(b64));
            match res {
                Ok(()) => {
                    spawn(async move {
                        let res = eval.recv().await;
                        match res {
                            Ok(serde_json::Value::String(string)) => {
                                let gateway = use_gateway();
                                let decode_res = base64::engine::general_purpose::STANDARD
                                    .decode(string)
                                    .ok()
                                    .and_then(|buffer| bincode::deserialize(&buffer).ok());
                                let rpc_res = match decode_res {
                                    Some(tx) => {
                                        log::info!("Sending: {:?}", tx);
                                        let x = gateway.rpc.send_transaction(&tx).await;
                                        log::info!("Sent: {:?}", x);
                                        x.ok()
                                    }
                                    None => {
                                        log::info!("error decoding tx");
                                        None
                                    }
                                };
                                log::info!("Dec: {:?}", rpc_res);
                                match rpc_res {
                                    Some(sig) => {
                                        log::info!("sig: {}", sig);
                                        let confirmed = gateway.confirm_signature(sig).await;
                                        if confirmed.is_ok() {
                                            signal.set(InvokeSignatureStatus::Done(sig));
                                        } else {
                                            signal.set(InvokeSignatureStatus::Timeout)
                                        }
                                    }
                                    None => {
                                        log::info!("error sending tx");
                                        signal.set(InvokeSignatureStatus::DoneWithError)
                                    }
                                }
                            }
                            _ => {
                                log::info!("err recv val");
                                signal.set(InvokeSignatureStatus::DoneWithError)
                            }
                        };
                    });
                }
                Err(_err) => {
                    log::info!("err sending val");
                    signal.set(InvokeSignatureStatus::DoneWithError)
                }
            }
        }
        Err(err) => {
            log::info!("err serializing tx: {}", err);
            signal.set(InvokeSignatureStatus::DoneWithError)
        }
    };
}

#[derive(PartialEq)]
pub enum InvokeSignatureStatus {
    Start,
    Waiting,
    DoneWithError,
    Timeout,
    Done(Signature),
}

#[derive(Clone)]
pub struct Balances {
    pub v1: UiTokenAmount,
    pub _v2: UiTokenAmount,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WalletAdapter {
    Connected(Pubkey),
    Disconnected,
}
