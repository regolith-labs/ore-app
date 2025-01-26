use base64::Engine;
use dioxus::{document::eval, prelude::*};
use solana_sdk::transaction::VersionedTransaction;

use crate::{
    components::*,
    gateway::{solana::SolanaGateway, Rpc},
    hooks::use_gateway,
};

pub fn invoke_signature(tx: VersionedTransaction, mut signal: Signal<InvokeSignatureStatus>) {
    spawn(async move {
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
                        let res = eval.recv().await;
                        match res {
                            Ok(serde_json::Value::String(string)) => {
                                let gateway = use_gateway();
                                let decode_res = base64::engine::general_purpose::STANDARD
                                    .decode(string)
                                    .ok();
                                let decode_res = decode_res.and_then(|buffer| {
                                    bincode::deserialize::<VersionedTransaction>(&buffer).ok()
                                });
                                let rpc_res = match decode_res {
                                    Some(tx) => gateway.rpc.send_transaction(&tx).await.ok(),
                                    None => {
                                        log::info!("error decoding tx");
                                        None
                                    }
                                };
                                match rpc_res {
                                    Some(sig) => {
                                        log::info!("sig: {}", sig);
                                        let confirmed = gateway.rpc.confirm_signature(sig).await;
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
    });
}
