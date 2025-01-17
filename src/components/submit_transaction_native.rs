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

#[component]
pub fn SubmitTransaction(
    tx: VersionedTransaction,
    signal: Signal<InvokeSignatureStatus>,
    start_msg: String,
) -> Element {
    let button_class = "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700";
    let error_class = "flex flex-row flex-nowrap gap-2 text-white w-min ml-auto text-nowrap bg-red-500 text-center font-semibold text-sm rounded py-1 px-2";
    rsx! {
        Col {
            gap: 6,
            if let InvokeSignatureStatus::DoneWithError = *signal.read() {
                p {
                    class: "{error_class}",
                    WarningIcon {
                        class: "w-3.5 h-3.5 my-auto"
                    }
                    "Transaction failed"
                }
            }
            if let InvokeSignatureStatus::Timeout = *signal.read() {
                p {
                    class: "{error_class}",
                    WarningIcon {
                        class: "w-3.5 h-3.5 my-auto"
                    }
                    "Timed out"
                }
            }

            // PriorityFeeConfig { signal }

            match *signal.read() {
                InvokeSignatureStatus::Start => {
                    rsx! {
                        button {
                            class: "{button_class}",
                            onclick: move |_| {
                                invoke_signature(tx.clone(), signal);
                            },
                            "{start_msg}"
                        }
                    }
                }
                InvokeSignatureStatus::Waiting => {
                    rsx! {
                        button {
                            class: "{button_class}",
                            disabled: true,
                            // Spinner { class: "mx-auto" }
                        }
                    }
                }
                InvokeSignatureStatus::DoneWithError | InvokeSignatureStatus::Timeout => {
                    rsx! {
                        button {
                            class: "{button_class}",
                            onclick: move |_| {
                                invoke_signature(tx.clone(), signal);
                            },
                            "Retry"
                        }
                    }
                }
                InvokeSignatureStatus::Done(_sig) => {
                    rsx! {
                        button {
                            class: "w-full py-3 rounded font-semibold text-white bg-green-500",
                            disabled: true,
                            CheckCircleIcon { class: "h-5 w-5 mx-auto" }
                        }
                    }
                }
            }
        }
    }
}

pub fn invoke_signature(tx: VersionedTransaction, mut signal: Signal<InvokeSignatureStatus>) {
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
