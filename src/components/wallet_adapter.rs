use dioxus::prelude::*;

use crate::components::*;
use crate::hooks::{invoke_signature, use_wallet_status, InvokeSignatureStatus, WalletStatus};
use crate::steel_app::solana::sdk::{pubkey::Pubkey, transaction::Transaction};
use crate::components::wallet_drawer::WalletDrawer;

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

    let mut drawer_open = use_signal(|| false);
    let drawer_container = if *drawer_open.read() { "bg-black/50" } else { "bg-transparent pointer-events-none" };
    let drawer_transform = if *drawer_open.read() { "translate-x-0" } else { "translate-x-full" };

    rsx! {
        div {
            class: "relative",
            button {
                onclick: move |_| {
                    drawer_open.set(!drawer_open.cloned());
                },
                Row {
                    class: "elevated-control elevated-border rounded-full text-sm font-semibold h-12 px-5 hover:cursor-pointer gap-3",
                    gap: 3,
                    span {
                        class: "mx-auto my-auto",
                        "{first_four}...{last_four}"
                    }
                    DrawerIcon {
                        class: "w-3 text-gray-700"
                    }
                }
            }
            div {
                class: "fixed inset-0 transition-colors duration-200 ease-in-out {drawer_container}",
                onclick: move |_| drawer_open.set(false),
                div {
                    class: "fixed top-0 right-0 h-full transition-transform duration-200 ease-in-out {drawer_transform}",
                    WalletDrawer {
                        on_close: move |_| drawer_open.set(false)
                    }
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
