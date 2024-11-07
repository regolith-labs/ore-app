use base64::Engine;
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    pubkey::Pubkey, signature::Signature, transaction::Transaction,
};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WalletAdapter {
    Connected(Pubkey),
    Disconnected,
}
