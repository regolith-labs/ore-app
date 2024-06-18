use dioxus::prelude::*;

use crate::{
    components::MinerToolbar,
    hooks::{
        use_miner_toolbar_state, use_miner_toolbar_state_provider, ReadMinerToolbarState,
        UpdateMinerToolbarState,
    },
    Route,
};

pub fn MinerToolbarLayout() -> Element {
    use_miner_toolbar_state_provider();
    let route = use_route::<Route>();
    let hidden = !matches!(
        route,
        Route::Home {} | Route::Tx { .. } | Route::User { .. }
    );

    rsx! {
        div {
            class: "flex flex-col h-full grow",
            Outlet::<Route> {}
        }
        ToolbarClose {
            hidden
        }
        MinerToolbar {
            hidden
        }
    }
}

#[component]
pub fn ToolbarClose(hidden: bool) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let opacity = if toolbar_state.is_open() {
        "opacity-80"
    } else {
        "opacity-0 pointer-events-none"
    };
    if hidden {
        return None;
    }
    rsx! {
        button {
            class: "fixed transition-opacity flex flex-row left-0 top-0 h-screen w-screen bg-black {opacity}",
            onclick: move |_e| {
                toolbar_state.set_is_open(false);
            }
        }
    }
}
