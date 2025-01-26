use base64::Engine;
use dioxus::{document::eval, prelude::*};
use solana_sdk::{message::VersionedMessage, transaction::VersionedTransaction};

use crate::{
    components::*,
    gateway::{solana::SolanaGateway, Rpc},
    hooks::use_gateway,
};

pub fn submit_transaction(mut tx: VersionedTransaction, mut signal: Signal<TransactionStatus>) {
    spawn(async move {
        // Set blockhash
        let gateway = use_gateway();
        if let Ok(hash) = gateway.rpc.get_latest_blockhash().await {
            match &mut tx.message {
                VersionedMessage::V0(message) => {
                    message.recent_blockhash = hash;
                }
                VersionedMessage::Legacy(message) => {
                    message.recent_blockhash = hash;
                }
            }
        }

        // Send for signing
        signal.set(TransactionStatus::Waiting);
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
                                            signal.set(TransactionStatus::Done(sig));
                                        } else {
                                            signal.set(TransactionStatus::Timeout)
                                        }
                                    }
                                    None => {
                                        log::info!("error sending tx");
                                        signal.set(TransactionStatus::Error)
                                    }
                                }
                            }
                            _ => {
                                log::info!("err recv val");
                                signal.set(TransactionStatus::Error)
                            }
                        };
                    }
                    Err(_err) => {
                        log::info!("err sending val");
                        signal.set(TransactionStatus::Error)
                    }
                }
            }
            Err(err) => {
                log::info!("err serializing tx: {}", err);
                signal.set(TransactionStatus::Error)
            }
        };
    });
}
