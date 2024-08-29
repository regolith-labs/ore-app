use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::transaction::Transaction;

use crate::components::WarningIcon;
use crate::components::{icons::CheckCircleIcon, Appearance, Spinner};
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
            class: "rounded-full transition-colors my-auto h-8 sm:h-10 {button_color}",
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
    let button_class = "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700";
    let error_class = "flex flex-row flex-nowrap gap-2 text-white w-min ml-auto text-nowrap bg-red-500 text-center font-semibold text-sm rounded py-1 px-2";
    rsx! {
        div {
            class: "flex flex-col gap-6",
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
                                use_wallet_adapter::invoke_signature(tx.clone(), signal);
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
                            Spinner { class: "mx-auto" }
                        }
                    }
                }
                InvokeSignatureStatus::DoneWithError | InvokeSignatureStatus::Timeout => {
                    // TODO: could add reset button here
                    // or other signal to user
                    rsx! {
                        button {
                            class: "{button_class}",
                            onclick: move |_| {
                                use_wallet_adapter::invoke_signature(tx.clone(), signal);
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
