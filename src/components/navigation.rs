use dioxus::prelude::*;

use crate::{
    components::{BeakerIcon, BoltIcon, GlobeIcon, OreWordmarkIcon, WalletAdapter},
    route::Route,
};

pub fn AppNavigation() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-full",
            AppNavBar {}
            MobileTabBar {}
            span {
                class: "py-5 sm:py-8 w-full max-w-[80rem] mx-auto",
                Outlet::<Route> {}
            }
        }
    }
}

pub fn LandingNavigation() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-screen",
            LandingNavBar {}
            Outlet::<Route> {}
        }
    }
}

fn AppNavBar() -> Element {
    rsx! {
        div {
            class: "flex w-screen h-16 sm:h-20 px-5 sm:px-8 sm:sticky sm:top-0 bg-black shadow",
            div {
                class: "flex flex-row justify-center sm:justify-between w-full my-auto",
                span {
                    class: "hidden sm:flex my-auto",
                    Logo {}
                }
                TabBar {}
                WalletAdapter {}
            }
        }
    }
}

fn LandingNavBar() -> Element {
    rsx! {
        div {
            class: "flex w-screen shrink-0 h-16 sm:h-20 px-5 sm:px-8",
            div {
                class: "flex flex-row w-full my-auto",
                Logo {}
            }
        }
    }
}

fn Logo() -> Element {
    rsx! {
        Link {
            class: "p-1 my-auto rounded hover:bg-gray-800",
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
            class: "hidden sm:flex flex-row h-full rounded-full elevated overflow-clip",
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
    let current_route = use_route();
    let selected = is_tab_selected(&route, &current_route);
    let color = if !selected { "text-gray-700" } else { "" };
    rsx! {
        Link {
            class: "flex px-8 h-10 transition hover:bg-gray-800",
            to: route,
            span {
                class: "text-sm font-medium my-auto font-semibold {color}",
                "{title}"
            }
        }
    }
}

fn MobileTabBar() -> Element {
    rsx! {
        div {
            class: "sm:hidden flex flex-row fixed bottom-0 w-full elevated",
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
    let selected = is_tab_selected(&route, &current_route);
    let color = if !selected { "text-gray-700" } else { "" };
    rsx! {
        Link {
            class: "flex h-16 w-full",
            to: route.clone(),
            span {
                class: "flex flex-col gap-1 mx-auto my-auto {color}",
                match route {
                    Route::Mine {  } => rsx!{
                        BoltIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    Route::Stake {  } => rsx!{
                        BeakerIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    Route::Trade {  } => rsx!{
                        GlobeIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    _ => rsx! {}
                }
                span {
                    class: "mx-auto font-medium text-xs",
                    "{title}"
                }
            }
        }
    }
}

fn is_tab_selected(route: &Route, current_route: &Route) -> bool {
    match route {
        Route::Mine {} => match current_route {
            Route::Mine {} | Route::Pool { pool: _ } => true,
            _ => false,
        },
        Route::Stake {} => match current_route {
            Route::Stake {} | Route::Pair { pair: _ } => true,
            _ => false,
        },
        Route::Trade {} => match current_route {
            Route::Trade {} | Route::Asset { asset: _ } => true,
            _ => false,
        },
        _ => false,
    }
}
