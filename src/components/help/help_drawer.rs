use crate::components::*;
use crate::hooks::use_help_drawer_state;
use dioxus::prelude::*;
use std::time::Duration;

pub fn HelpDrawer() -> Element {
    let mut drawer_state = use_help_drawer_state();
    let drawer_remount = use_signal(|| false);
    let mut is_animating = use_signal(|| false);

    // Only render the drawer, not the button
    rsx! {
        HelpDrawerOverlay {
            drawer_state: drawer_state,
            is_animating: is_animating,
            on_close: move |_| {
                if !is_animating.cloned() {
                    is_animating.set(true);
                    let mut current = drawer_state.read().clone();
                    current.is_open = false;
                    drawer_state.set(current);
                    spawn(async move {
                        async_std::task::sleep(Duration::from_millis(500)).await;
                        is_animating.set(false);
                    });
                }
            },
            drawer_remount: drawer_remount
        }
    }
}

#[component]
fn HelpDrawerOverlay(
    drawer_state: Signal<crate::hooks::HelpDrawerState>,
    is_animating: Signal<bool>,
    on_close: EventHandler<MouseEvent>,
    drawer_remount: Signal<bool>,
) -> Element {
    let is_open = drawer_state.read().is_open;

    // Render nothing when closed and not animating
    if !is_open && !*is_animating.read() {
        return rsx! { Fragment {} };
    }

    // Render drawer when open or animating
    rsx! {
        Fragment {
            // Background overlay with fade effect
            div {
                class: "fixed inset-0 transition-all duration-500 ease-in-out bg-black/50 blur-md z-50",
                class: if is_open { "wallet-drawer-fade opacity-100" } else { "wallet-drawer-fade-out opacity-0" },
                onclick: move |e| on_close(e)
            }
            // Drawer content
            div {
                class: "fixed top-0 right-0 h-full w-screen sm:w-[574px] transition-transform duration-500 ease-in-out transform z-[1001] scrollbar-hide",
                class: if is_open { "wallet-drawer-slide translate-x-0" } else { "wallet-drawer-slide-out translate-x-full" },
                style: "height: 100vh;",
                HelpDrawerWrapper {
                    drawer_state: drawer_state,
                    on_close: on_close.clone(),
                    drawer_remount: drawer_remount
                }
            }
        }
    }
}
