use dioxus::prelude::*;

use crate::components::{CarrotDownIcon, CheckCircleIcon, Col, Row, WarningIcon};
use crate::hooks::{invoke_signature, use_wallet_status, InvokeSignatureStatus, WalletStatus};
use crate::steel_app::solana::sdk::{pubkey::Pubkey, transaction::Transaction};

pub fn WalletAdapter() -> Element {
    let wallet_status = use_wallet_status();

    let mut wallet_mount = use_future(move || async move {
        let eval = eval(
            r#"
                window.MountWalletAdapter();
                return
            "#,
        );
        let _ = eval.await;
    });

    let mut wallet_remount = use_signal(|| false);

    match wallet_status.cloned() {
        WalletStatus::Connected(address) => {
            rsx! {
                ConnectedWalletAdapter {
                    address: address,
                    wallet_remount
                }
            }
        }
        WalletStatus::Disconnected => {
            if *wallet_remount.read() {
                wallet_mount.restart();
                wallet_remount.set(false);
            }
            rsx! {
                div {
                    class: "rounded-full transition-colors my-auto h-10 text-black bg-white",
                    nav {
                        id: "ore-wallet-adapter"
                    }
                }
            }
        }
    }
}

#[component]
fn ConnectedWalletAdapter(address: Pubkey, wallet_remount: Signal<bool>) -> Element {
    let len = address.to_string().len();
    let first_four = &address.to_string()[0..4];
    let last_four = &address.to_string()[len - 4..len];

    rsx! {
        button {
            onclick: move |_| {
                wallet_remount.set(true);
                let disconnect = eval(
                    r#"
                    window.OreWalletDisconnecter();
                    return
                "#,
                );
                spawn(async move {
                    disconnect.await;
                });
            },
            Row {
                class: "elevated-control elevated-border rounded-full text-sm font-semibold h-10 px-4 hover:cursor-pointer",
                gap: 2,
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
