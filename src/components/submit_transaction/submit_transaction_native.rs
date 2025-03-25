use dioxus::prelude::*;
use ore_types::request::TransactionType;
use solana_sdk::{hash::Hash, signature::Keypair, transaction::VersionedTransaction};

use crate::{
    components::*,
    gateway::{solana::SolanaGateway, GatewayResult, NativeRpc, Rpc},
    hooks::{use_gateway, use_transaction_status},
};

/// sign transactions without necessarily submitting them,
/// useful for things like posting signed transactions to servers.
pub async fn sign_transaction(
    tx: VersionedTransaction,
) -> GatewayResult<(VersionedTransaction, Hash)> {
    let gateway = use_gateway();
    let signer = crate::hooks::use_wallet_native::get()?;
    sign(&gateway.rpc, &signer.creator, tx).await
}

pub async fn sign_transaction_partial(
    tx: VersionedTransaction,
    second_signer: SecondSigner,
) -> GatewayResult<(VersionedTransaction, Hash)> {
    let gateway = use_gateway();
    let signer = crate::hooks::use_wallet_native::get()?;
    let hash = gateway.rpc.get_latest_blockhash().await?;
    let mut message = tx.message;
    message.set_recent_blockhash(hash);
    let present = MixedSigners::Present(signer.creator);
    let absent = MixedSigners::Absent(second_signer.signer);
    let signed = match second_signer.payer {
        true => VersionedTransaction::try_new(message, &[&absent, &present])?,
        false => VersionedTransaction::try_new(message, &[&present, &absent])?,
    };
    Ok((signed, hash))
}

pub fn submit_transaction(tx: VersionedTransaction, _tx_type: TransactionType) {
    let mut transaction_status = use_transaction_status();
    spawn(async move {
        transaction_status.set(Some(TransactionStatus::Waiting));
        // get signer
        match crate::hooks::use_wallet_native::get() {
            Ok(signer) => {
                let gateway = use_gateway();
                transaction_status.set(Some(TransactionStatus::Sending(0)));
                // sign
                if let Err(err) = sign_submit_confirm(&gateway.rpc, &signer.creator, tx).await {
                    log::error!("{:?}", err);
                    transaction_status.set(Some(TransactionStatus::Error));
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                transaction_status.set(Some(TransactionStatus::Denied));
            }
        }
    });
}

async fn sign(
    rpc: &NativeRpc,
    signer: &Keypair,
    tx: VersionedTransaction,
) -> GatewayResult<(VersionedTransaction, Hash)> {
    let hash = rpc.get_latest_blockhash().await?;
    let mut message = tx.message;
    message.set_recent_blockhash(hash);
    let signed = VersionedTransaction::try_new(message, &[signer])?;
    Ok((signed, hash))
}

async fn sign_submit_confirm(
    rpc: &NativeRpc,
    signer: &Keypair,
    tx: VersionedTransaction,
) -> GatewayResult<()> {
    let mut transaction_status = use_transaction_status();
    // sign
    let (signed, _) = sign(rpc, signer, tx).await?;
    // submit
    let sig = rpc.send_transaction(&signed).await?;
    // confirm
    let confirmed = rpc.confirm_signature(sig).await;
    if confirmed.is_ok() {
        transaction_status.set(Some(TransactionStatus::Done(sig)));
    } else {
        transaction_status.set(Some(TransactionStatus::Timeout));
    }
    Ok(())
}
