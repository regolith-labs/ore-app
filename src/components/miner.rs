use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{IsToolbarOpen, MinerToolbar},
    Route,
};

#[component]
pub fn Miner(cx: Scope) -> Element {
    use_shared_state_provider(cx, || IsToolbarOpen(false));
    let route = use_route::<Route>(cx);

    let hidden = if let Some(route) = route {
        !matches!(
            route,
            Route::Home {} | Route::Tx { .. } | Route::User { .. }
        )
    } else {
        true
    };

    render! {
        div {
            class: "flex flex-col h-full grow",
            Outlet::<Route> {}
        }
        ToolbarClose {
            hidden: hidden
        }
        MinerToolbar {
            hidden: hidden
        }
    }
}

#[component]
pub fn ToolbarClose(cx: Scope, hidden: bool) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let opacity = if is_toolbar_open.read().0 {
        "opacity-80"
    } else {
        "opacity-0 pointer-events-none"
    };
    if *hidden {
        return None;
    }
    render! {
        button {
            class: "fixed transition-opacity flex flex-row left-0 top-0 h-screen w-screen bg-black {opacity}",
            onclick: |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(false);
            }
        }
    }
}
