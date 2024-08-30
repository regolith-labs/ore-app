use dioxus::prelude::*;

use crate::{components::MinerToolbar, hooks::use_miner_toolbar_state_provider, Route};

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
        MinerToolbar {
            hidden
        }
    }
}
