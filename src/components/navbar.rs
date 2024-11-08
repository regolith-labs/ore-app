use dioxus::prelude::*;

use crate::{
    components::{OreWordmarkIcon, WalletAdapter},
    route::Route,
};

pub fn NavBarLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-full",
            NavBar {}
            MobileTabBar {}
            Outlet::<Route> {}
        }
    }
}

fn NavBar() -> Element {
    rsx! {
        div {
            class: "flex w-screen h-20 px-5 sm:px-8",
            div {
                class: "flex flex-row justify-end sm:justify-between w-full my-auto",
                Logo {}
                TabBar {}
                WalletAdapter {}
            }
        }
    }
}

fn Logo() -> Element {
    rsx! {
        Link {
            class: "hidden sm:flex my-auto",
            to: Route::Landing {},
            OreWordmarkIcon {
                class: "h-5"
            }
        }
    }
}

fn TabBar() -> Element {
    rsx! {
        div {
            class: "hidden sm:flex flex-row h-full",
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
            class: "flex px-8 h-10",
            to: route,
            span {
                class: "text-sm font-medium my-auto {opacity}",
                "{title}"
            }
        }
    }
}

fn MobileTabBar() -> Element {
    rsx! {
        div {
            class: "flex flex-row fixed bottom-0 w-full block sm:hidden",
            MobileTab {
                title: "Mine",
                route: Route::Mine {}
            }
            MobileTab {
                title: "Stake",
                route: Route::Stake {}
            }
            MobileTab {
                title: "Trade",
                route: Route::Trade {}
            }
        }
    }
}

#[component]
fn MobileTab(title: String, route: Route) -> Element {
    let current_route: Route = use_route();
    let opacity = if route != current_route {
        "opacity-50"
    } else {
        ""
    };
    rsx! {
        Link {
            class: "flex h-20 w-full",
            to: route,
            span {
                class: "flex flex-col gap-1 mx-auto my-auto {opacity}",
                span {
                    class: "h-8 w-8 mx-auto bg-black"
                }
                span {
                    class: "mx-auto text-xs",
                    "{title}"
                }
            }
        }
    }
}
