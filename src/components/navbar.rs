use dioxus::prelude::*;

use crate::{components::WalletAdapter, route::Route};

pub fn NavbarLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-full",
            Navbar {}
            Outlet::<Route> {}
        }
    }
}

fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-16 w-full",
            Logo {}
            Tabs {}
            WalletAdapter {}
        }
    }
}

fn Logo() -> Element {
    rsx! {
        Link {
            to: Route::Landing {},
            "ORE"
        }
    }
}

fn Tabs() -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full gap-8",
            Tab {
                title: "Mine",
                route: Route::Mine {}
            }
            Tab {
                title: "Stake",
                route: Route::Stake {}
            }
            Tab {
                title: "Trade",
                route: Route::Trade {}
            }
        }
    }
}

#[component]
fn Tab(title: String, route: Route) -> Element {
    let current_route: Route = use_route();
    let opacity = if route != current_route {
        "opacity-50"
    } else {
        ""
    };
    rsx! {
        Link {
            class: "{opacity}",
            to: route,
            "{title}"
        }
    }
}
