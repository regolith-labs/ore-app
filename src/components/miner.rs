use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{IsModalOpen, IsToolbarOpen, MinerToolbar},
    hooks::use_is_onboarded,
    Route,
};

#[component]
pub fn Miner(cx: Scope) -> Element {
    use_shared_state_provider(cx, || IsToolbarOpen(false));
    use_shared_state_provider(cx, || IsModalOpen(false));
    let is_onboarded = use_is_onboarded(cx);
    let route = use_route::<Route>(cx);

    // If the user is not onboarded, redirect to Landing.
    use_effect(cx, (), |_| {
        is_onboarded.set(true);
        async move {}
    });

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
            class: "flex flex-col h-full grow dark",
            Outlet::<Route> {}
        }
        ToolbarClose {}
        MinerToolbar {
            hidden: hidden
        }
    }
}

#[component]
pub fn ToolbarClose(cx: Scope) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let opacity = if is_toolbar_open.read().0 {
        "opacity-80"
    } else {
        "opacity-0 pointer-events-none"
    };
    render! {
        button {
            class: "absolute transition-opacity flex flex-row left-0 top-0 h-screen w-screen bg-black {opacity}",
            onclick: |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(false);
            }
        }
    }
}
