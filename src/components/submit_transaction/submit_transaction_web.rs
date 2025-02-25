use base64::Engine;
use dioxus::{document::eval, prelude::*};
use js_sys::Date;
use ore_types::request::{AppId, TransactionEvent, TransactionType};
use solana_sdk::{message::VersionedMessage, transaction::VersionedTransaction};

use crate::{
    components::*,
    gateway::{ore::OreGateway, solana::SolanaGateway, Rpc},
    hooks::{use_gateway, use_transaction_status},
};

pub fn submit_transaction(mut tx: VersionedTransaction, tx_type: TransactionType) {
    let mut transaction_status = use_transaction_status();

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

        // Build eval command for wallet signing
        let mut eval = eval(
            r#"
            let msg = await dioxus.recv();
            let signed = await window.OreTxSigner({b64: msg});
            dioxus.send(signed);
            "#,
        );

        // Serialized the transaction to send to wallet
        match bincode::serialize(&tx) {
            Ok(vec) => {
                transaction_status.set(Some(TransactionStatus::Waiting));
                let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
                let res = eval.send(serde_json::Value::String(b64));
                match res {
                    Ok(()) => {
                        // Execute eval command
                        let res = eval.recv().await;

                        // Process eval result
                        match res {
                            // Process valid signing result
                            Ok(serde_json::Value::String(string)) => {
                                // Decode signed transaction
                                let gateway = use_gateway();
                                let decode_res = base64::engine::general_purpose::STANDARD
                                    .decode(string)
                                    .ok();
                                let decode_res = decode_res.and_then(|buffer| {
                                    bincode::deserialize::<VersionedTransaction>(&buffer).ok()
                                });

                                // Send transaction to rpc
                                transaction_status.set(Some(TransactionStatus::Sending(0)));
                                let rpc_res = match decode_res {
                                    Some(tx) => gateway.rpc.send_transaction(&tx).await.ok(),
                                    None => {
                                        log::info!("error decoding tx");
                                        None
                                    }
                                };

                                let signer = tx.message.static_account_keys()[0];
                                let timestamp = (Date::now() / 1000.0) as i64;
                                // Write transaction to db (API)
                                if let Some(sig) = rpc_res {
                                    match gateway
                                        .log_transaction_db(TransactionEvent {
                                            sig: sig,
                                            signer: signer,
                                            transaction_type: tx_type,
                                            app: AppId::OreWeb,
                                            ts: timestamp,
                                            status: None,
                                            fee: None,
                                        })
                                        .await
                                    {
                                        Ok(_sig) => {}
                                        Err(e) => {
                                            log::error!("Error writing transaction to db: {:?}", e);
                                        }
                                    }
                                }

                                // Confirm transaction
                                match rpc_res {
                                    Some(sig) => {
                                        let confirmed = gateway.rpc.confirm_signature(sig).await;
                                        if confirmed.is_ok() {
                                            transaction_status
                                                .set(Some(TransactionStatus::Done(sig)));
                                        } else {
                                            transaction_status
                                                .set(Some(TransactionStatus::Timeout));
                                        }
                                    }
                                    None => {
                                        log::info!("error sending tx");
                                        transaction_status.set(Some(TransactionStatus::Error))
                                    }
                                }
                            }

                            // Process signing errors
                            Ok(serde_json::Value::Null) => {
                                transaction_status.set(Some(TransactionStatus::Denied))
                            }
                            Err(err) => {
                                log::error!("error signing transaction: {}", err);
                                transaction_status.set(Some(TransactionStatus::Error))
                            }
                            _ => {
                                log::error!("unrecognized signing response");
                                transaction_status.set(Some(TransactionStatus::Error))
                            }
                        };
                    }

                    // Process eval errors
                    Err(err) => {
                        log::error!("error executing wallet signing script: {}", err);
                        transaction_status.set(Some(TransactionStatus::Error))
                    }
                }
            }

            // Process serialization errors
            Err(err) => {
                log::error!("err serializing tx: {}", err);
                transaction_status.set(Some(TransactionStatus::Error))
            }
        };
    });
}
