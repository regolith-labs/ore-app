use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::components::*;
use crate::hooks::{use_wallet, use_wallet_drawer_state, Wallet};
use crate::time::Duration;

pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();

    let mut wallet_mount = use_future(move || async move {
        if Wallet::Disconnected == wallet.cloned() {
            async_std::task::sleep(Duration::from_millis(500)).await;
            let eval = eval(
                r#"
                    window.MountWalletAdapter();
                    return
                "#,
            );
            let _ = eval.await;
        }
    });

    let mut wallet_remount = use_signal(|| false);

    match wallet.cloned() {
        Wallet::Connected(address) => {
            rsx! {
                ConnectedWalletAdapter {
                    address: address,
                    wallet_remount
                }
            }
        }
        Wallet::Disconnected => {
            if *wallet_remount.peek() {
                wallet_mount.restart();
                wallet_remount.set(false);
            }
            rsx! {
                div {
                    class: "rounded-full transition my-auto h-12 text-black bg-white hover:cursor-pointer hover:scale-105 duration-300 ease-in-out bg-controls-primary",
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

    let mut drawer_state = use_wallet_drawer_state();
    let is_open = *drawer_state.read();

    rsx! {
        div {
            class: "relative",
            button {
                onclick: move |_| {
                    drawer_state.set(!is_open);
                },
                Row {
                    class: "elevated-control elevated-border rounded-full text-sm font-semibold h-12 px-5 hover:cursor-pointer gap-3",
                    gap: 3,
                    span {
                        class: "mx-auto my-auto",
                        "{first_four}...{last_four}"
                    }
                    DrawerIcon {
                        class: "w-3 text-elements-lowEmphasis"
                    }
                }
            }

            // Drawer overlay and content
            WalletDrawerOverlay {
                is_open: is_open,
                on_close: move |_| drawer_state.set(false),
                wallet_remount: wallet_remount
            }
        }
    }
}

#[component]
fn WalletDrawerOverlay(
    is_open: bool,
    on_close: EventHandler<MouseEvent>,
    wallet_remount: Signal<bool>,
) -> Element {
    // Render nothing when closed
    if !is_open {
        return rsx! { Fragment {} };
    }

    // Render drawer when open
    rsx! {
        Fragment {
            // Only show dark backdrop overlay on mobile (sm:hidden)
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 z-[1000] sm:hidden",
                style: "height: 100vh; width: 100vw;",
                onclick: move |e| on_close.call(e)
            }
            // Invisible overlay for desktop to capture clicks outside (hidden on mobile)
            div {
                class: "fixed inset-0 z-[1000] hidden sm:block",
                style: "height: 100vh; width: 100vw;",
                onclick: move |e| on_close.call(e)
            }
            div {
                class: "fixed top-0 right-0 bottom-0 h-full w-screen sm:w-96 z-[1001]",
                style: "height: 100vh;",
                WalletDrawer {
                    on_close: on_close.clone(),
                    wallet_remount: wallet_remount
                }
            }
        }
    }
}
