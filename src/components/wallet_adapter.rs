use dioxus::prelude::*;

use crate::components::{CarrotDownIcon, CheckCircleIcon, WarningIcon};
use crate::hooks::{invoke_signature, use_wallet_status, InvokeSignatureStatus, WalletStatus};
use crate::steel_app::solana::sdk::{pubkey::Pubkey, transaction::Transaction};

pub fn WalletAdapter() -> Element {
    let wallet_status = use_wallet_status();

    let _ = use_future(move || async move {
        let eval = eval(
            r#"
                window.MountWalletAdapter();
                return
            "#,
        );
        let _ = eval.await;
    });

    match wallet_status.cloned() {
        WalletStatus::Connected(address) => {
            rsx! {
                ConnectedWalletAdapter {
                    address: address
                }
            }
        }
        WalletStatus::Disconnected => {
            rsx! {
                div {
                    class: "rounded-full transition-colors my-auto h-8 sm:h-10 text-black bg-white",
                    nav {
                        id: "ore-wallet-adapter"
                    }
                }
            }
        }
    }
}

// TODO Disconnect options

#[component]
fn ConnectedWalletAdapter(address: Pubkey) -> Element {
    let len = address.to_string().len();
    let first_four = &address.to_string()[0..4];
    let last_four = &address.to_string()[len - 4..len];

    rsx! {
        div {
            class: "flex flex-row gap-2 elevated rounded-full text-sm font-semibold h-8 sm:h-10 px-4 transition hover:cursor-pointer hover:bg-gray-800",
            span {
                class: "mx-auto my-auto",
                "{first_four}...{last_four}"
            }
            CarrotDownIcon {
                class: "w-3 text-gray-700"
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
                    // TODO: could add reset button here
                    // or other signal to user
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
