use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::components::*;
use crate::hooks::{use_help_drawer_state, use_wallet, Wallet};
use crate::time::Duration;

pub fn HelpDrawerMount() -> Element {
    let mut drawer_remount = use_signal(|| false);

    rsx! {
        HelpDrawerAdapter {
            drawer_remount
        }
    }
}

#[component]
fn HelpDrawerAdapter(drawer_remount: Signal<bool>) -> Element {
    let mut drawer_state = use_help_drawer_state();
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
                        "Help"
                    }
                    DrawerIcon {
                        class: "w-3 text-gray-700"
                    }
                }
            }

            // Drawer overlay and content
            HelpDrawerOverlay {
                is_open: is_open,
                on_close: move |_| drawer_state.set(false),
                drawer_remount: drawer_remount
            }
        }
    }
}

#[component]
fn HelpDrawerOverlay(
    is_open: bool,
    on_close: EventHandler<MouseEvent>,
    drawer_remount: Signal<bool>,
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
                class: "fixed top-0 right-0 bottom-0 h-full w-screen sm:w-120 z-[1001]",
                style: "height: 100vh;",
                HelpDrawer {
                    on_close: on_close.clone(),
                    drawer_remount: drawer_remount
                }
            }
        }
    }
}
