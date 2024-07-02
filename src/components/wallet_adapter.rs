use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::transaction::Transaction;

use crate::components::Spinner;
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
pub fn InvokeSignature(
    tx: Transaction,
    signal: Signal<InvokeSignatureStatus>,
    start_msg: String,
) -> Element {
    let e = match *signal.read() {
        InvokeSignatureStatus::Start => {
            rsx! {
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                    onclick: move |_| {
                        use_wallet_adapter::invoke_signature(tx.clone(), signal);
                    },
                    "{start_msg}"
                }
            }
        }
        InvokeSignatureStatus::Waiting => {
            rsx! {
                Spinner { class: "mx-auto" }
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
                    class: "mx-auto",
                    "{sig.to_string()}"
                }
            }
        }
    };
    e
}
