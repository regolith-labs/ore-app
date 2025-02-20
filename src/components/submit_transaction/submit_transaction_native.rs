use dioxus::prelude::*;
use solana_sdk::{
    signature::{Keypair, Signature},
    transaction::VersionedTransaction,
};

use crate::{
    components::*,
    gateway::{solana::SolanaGateway, GatewayResult, NativeRpc, Rpc},
    hooks::{use_gateway, use_transaction_status},
};

pub fn submit_transaction(tx: VersionedTransaction) {
    let mut transaction_status = use_transaction_status();
    spawn(async move {
        transaction_status.set(Some(TransactionStatus::Waiting));
        // get signer
        match crate::hooks::use_wallet_native::get() {
            Ok(signer) => {
                let gateway = use_gateway();
                transaction_status.set(Some(TransactionStatus::Sending(0)));
                // sign
                match sign_submit_confirm(&gateway.rpc, &signer.creator, tx).await {
                    Ok(sig) => {
                        transaction_status.set(Some(TransactionStatus::Done(sig)));
                        return;
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                        transaction_status.set(Some(TransactionStatus::Error));
                    }
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                transaction_status.set(Some(TransactionStatus::Denied));
                return;
            }
        }
    });
}

async fn sign_submit_confirm(
    rpc: &NativeRpc,
    signer: &Keypair,
    tx: VersionedTransaction,
) -> GatewayResult<Signature> {
    let hash = rpc.get_latest_blockhash().await?;
    let mut message = tx.message;
    message.set_recent_blockhash(hash);
    let signed = VersionedTransaction::try_new(message, &[signer])?;
    let sig = rpc.send_transaction(&signed).await?;
    let sig = rpc.confirm_signature(sig).await?;
    Ok(sig)
}
