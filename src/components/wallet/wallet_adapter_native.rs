use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::components::*;
use crate::hooks::{use_wallet, Wallet};

pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();
    match wallet.cloned() {
        Wallet::Connected(address) => {
            rsx! {
                ConnectedWalletAdapter {
                    address: address,
                }
            }
        }
        Wallet::Disconnected => {
            rsx! {
                div {
                    class: "rounded-full transition-colors my-auto h-10 text-black bg-white",
                }
            }
        }
    }
}

#[component]
fn ConnectedWalletAdapter(address: Pubkey) -> Element {
    let len = address.to_string().len();
    let first_four = &address.to_string()[0..4];
    let last_four = &address.to_string()[len - 4..len];

    let mut drawer_open = use_signal(|| false);
    let drawer_container = if *drawer_open.read() {
        "bg-black/50"
    } else {
        "bg-transparent pointer-events-none"
    };
    let drawer_transform = if *drawer_open.read() {
        "translate-x-0"
    } else {
        "translate-x-full"
    };

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
                        on_close: move |_| drawer_open.set(false),
                    }
                }
            }
        }
    }
}
