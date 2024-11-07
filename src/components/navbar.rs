use dioxus::prelude::*;

use crate::{components::WalletAdapter, route::Route};

pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full gap-16",
            "ORE"
            Tabs {}
            WalletAdapter {}
        }
    }
}

fn Tabs() -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-8",
            Link {
                to: Route::Mine {},
                "Mine"
            }
            Link {
                to: Route::Stake {},
                "Stake"
            }
            Link {
                to: Route::Trade {},
                "Trade"
            }
        }
    }
}
