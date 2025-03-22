use dioxus::prelude::*;

use crate::{
    components::{HelpDrawer, InfoIcon},
    hooks::{use_help_drawer_state, use_wallet_drawer_state},
};

#[component]
pub fn HelpAdapter() -> Element {
    let mut help_drawer_state = use_help_drawer_state();
    let mut wallet_drawer_state = use_wallet_drawer_state();
    let drawer_open = *help_drawer_state.read();

    rsx! {
        // Help button
        button {
            class: "flex items-center justify-center w-8 h-8 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis hover:bg-surface-floating transition-colors",
            onclick: move |_| {
                // Close wallet drawer if open
                if *wallet_drawer_state.read() {
                    wallet_drawer_state.set(false);
                }
                // Toggle help drawer
                help_drawer_state.set(!drawer_open);
            },
            InfoIcon {
                class: "h-4 w-4"
            }
        }

        // Show drawer overlay when open
        {
            drawer_open.then(|| rsx! {
                HelpDrawerOverlay {
                    on_close: move |_| help_drawer_state.set(false)
                }
            })
        }
    }
}

// Separate component for the Help Drawer Overlay so it can be used independently
#[component]
pub fn HelpDrawerOverlay(on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        // Simple full screen overlay that acts as a backdrop
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 z-[1000]",
            style: "height: 100vh; width: 100vw;",
            onclick: move |e| on_close.call(e),

            // Help drawer container
            div {
                class: "fixed top-0 right-0 h-full z-[1001]",
                onclick: move |e| e.stop_propagation(),

                // Actual help drawer component
                HelpDrawer {
                    on_close: on_close.clone()
                }
            }
        }
    }
}
