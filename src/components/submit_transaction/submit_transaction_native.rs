use dioxus::prelude::*;
use solana_sdk::{
    message::VersionedMessage,
    signature::{Keypair, Signature},
    transaction::VersionedTransaction,
};

use crate::{
    components::*,
    gateway::{solana::SolanaGateway, GatewayResult, NativeRpc, Rpc},
    hooks::{use_gateway, use_wallet},
};

pub fn submit_transaction(tx: VersionedTransaction, mut signal: Signal<InvokeSignatureStatus>) {
    spawn(async move {
        signal.set(InvokeSignatureStatus::Waiting);
        // get signer
        match crate::hooks::use_wallet_native::get() {
            Ok(signer) => {
                let gateway = use_gateway();
                // sign
                match sign_submit_confirm(&gateway.rpc, &signer.creator, tx).await {
                    Ok(sig) => {
                        signal.set(InvokeSignatureStatus::Done(sig));
                        return;
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
            }
        }
        signal.set(InvokeSignatureStatus::DoneWithError);
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
