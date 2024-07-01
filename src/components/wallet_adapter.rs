use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::transaction::Transaction;

use crate::hooks::{use_wallet_adapter, use_wallet_adapter::InvokeSignatureStatus};

#[component]
pub fn MountWalletAdapter() -> Element {
    let _ = use_future(move || async move {
        let eval = eval(
            r#"
                window.MountWalletAdapter();
                return
            "#,
        );
        let _ = eval.await;
    });
    rsx!(nav {
        id: "ore-wallet-adapter"
    })
}

#[component]
pub fn InvokeSignature(tx: Transaction) -> Element {
    let signal = use_signal(|| InvokeSignatureStatus::Start);
    let e = match *signal.read() {
        InvokeSignatureStatus::Start => {
            rsx! {
                button {
                    onclick: move |_| {
                        use_wallet_adapter::invoke_signature(tx.clone(), signal);
                    }
                }
            }
        }
        InvokeSignatureStatus::Waiting => {
            rsx! {
                div {
                    "waiting"
                }
            }
        }
        InvokeSignatureStatus::DoneWithError => {
            // TODO: could add reset button here
            // or other signal to user
            rsx! {
                div {
                    "failed"
                }
            }
        }
        InvokeSignatureStatus::Done(sig) => {
            rsx! {
                div {
                    "success"
                }
                div {
                    "{sig.to_string()}"
                }
            }
        }
    };
    e
}
