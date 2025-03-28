use dioxus::prelude::*;

use crate::components::*;
use crate::hooks::{use_help_drawer_state, HelpDrawerPage};
use crate::route::Route;

pub fn HelpDrawer() -> Element {
    let mut drawer_state = use_help_drawer_state();
    let drawer_remount = use_signal(|| false);

    // Get the current route to determine if we should show the drawer
    let current_route: Route = use_route();

    // Only render the drawer, not the button
    rsx! {
        HelpDrawerOverlay {
            drawer_state: drawer_state,
            on_close: move |_| {
                let mut current = drawer_state.read().clone();
                current.is_open = false;
                drawer_state.set(current);
            },
            drawer_remount: drawer_remount
        }
    }
}

#[component]
fn HelpDrawerOverlay(
    drawer_state: Signal<crate::hooks::HelpDrawerState>,
    on_close: EventHandler<MouseEvent>,
    drawer_remount: Signal<bool>,
) -> Element {
    // Render nothing when closed
    if !drawer_state.read().is_open {
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
                class: "fixed top-0 right-0 bottom-0 h-full w-screen sm:w-[574px] z-[1001]",
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
