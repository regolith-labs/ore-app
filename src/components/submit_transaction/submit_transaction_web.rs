use base64::Engine;
use dioxus::{document::eval, prelude::*};
use js_sys::Date;
use ore_types::request::{AppId, TransactionEvent, TransactionType};
use solana_sdk::{
    hash::Hash,
    message::VersionedMessage,
    instruction::InstructionError,
    transaction::{Transaction, VersionedTransaction, TransactionError},
};

use crate::{
    components::*,
    gateway::{ore::OreGateway, solana::SolanaGateway, GatewayResult, Rpc, GatewayError},
    hooks::{use_gateway, use_transaction_status},
};

pub async fn sign_transaction_partial(mut tx: Transaction) -> GatewayResult<(Transaction, Hash)> {
    // set blockhash
    let gateway = use_gateway();
    let hash = gateway.rpc.get_latest_blockhash().await?;
    let message = &mut tx.message;
    message.recent_blockhash = hash;
    // build eval command for wallet signing
    let mut eval = eval(
        r#"
        let msg = await dioxus.recv();
        let signed = await window.OreTxSigner({b64: msg});
        dioxus.send(signed);
        "#,
    );
    // serialize transaction to send to wallet
    let vec = bincode::serialize(&tx).map_err(|err| anyhow::anyhow!(err))?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
    let _send = eval
        .send(serde_json::Value::String(b64))
        .map_err(|err| anyhow::anyhow!(err))?;
    // wait on eval
    let res = eval.recv().await;
    // process eval result
    if let Ok(serde_json::Value::String(string)) = res {
        // decode b64 signed transaction
        let buffer = base64::engine::general_purpose::STANDARD
            .decode(string)
            .map_err(|err| anyhow::anyhow!(err))?;
        // deserialize binary to transaction
        let tx =
            bincode::deserialize::<Transaction>(&buffer).map_err(|err| anyhow::anyhow!(err))?;
        Ok((tx, hash))
    } else {
        Err(anyhow::anyhow!("unexpected response format").into())
    }
}

/// signs and submits
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

                                let tx_for_simulation = tx.clone();                                

                                // Simulate transaction to check for insufficient funds
                                if let Ok(simulated_tx) = gateway.rpc.simulate_transaction(&tx_for_simulation).await {
                                    if let Some(err) = simulated_tx.err {
                                        if let TransactionError::InstructionError(index, instruction_error) = err {
                                            if matches!(instruction_error, InstructionError::Custom(1)) {
                                                transaction_status.set(Some(TransactionStatus::InsufficientFunds));
                                                return;
                                            }
                                        }
                                    }
                                }                                

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
                                        .log_transaction_event(TransactionEvent {
                                            sig,
                                            signer,
                                            transaction_type: tx_type,
                                            app: if cfg!(feature = "web") {
                                                AppId::OreWeb
                                            } else {
                                                AppId::OreDesktop
                                            },
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
