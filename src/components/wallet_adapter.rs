use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::transaction::Transaction;

use crate::components::{Appearance, Spinner};
use crate::hooks::{use_appearance, use_wallet_adapter, use_wallet_adapter::InvokeSignatureStatus};

#[component]
pub fn MountWalletAdapter() -> Element {
    let appearance = use_appearance();
    let wallet_adapter = use_wallet_adapter::use_wallet_adapter();
    let button_color = match *wallet_adapter.read() {
        use_wallet_adapter::WalletAdapter::Connected(_) => match *appearance.read() {
            Appearance::Light => "text-black hover:bg-gray-100 active:bg-gray-200",
            Appearance::Dark => "text-white hover:bg-gray-900 active:bg-gray-800",
        },
        use_wallet_adapter::WalletAdapter::Disconnected => {
            "text-white bg-green-500 hover:bg-green-600 active:bg-green-700"
        }
    };

    let _ = use_future(move || async move {
        let eval = eval(
            r#"
                window.MountWalletAdapter();
                return
            "#,
        );
        let _ = eval.await;
    });

    rsx! {
        div {
            class: "rounded-full transition-colors {button_color}",
            nav {
                id: "ore-wallet-adapter"
            }
        }
    }
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
