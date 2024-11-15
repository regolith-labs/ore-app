use dioxus::prelude::*;

use crate::{
    components::{BeakerIcon, BoltIcon, Col, GlobeIcon, OreWordmarkIcon, Row, WalletAdapter},
    route::Route,
};

pub fn AppNavigation() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full",
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
        Col {
            class: "w-screen h-screen",
            LandingNavBar {}
            Outlet::<Route> {}
        }
    }
}

fn AppNavBar() -> Element {
    rsx! {
        Row {
            class: "w-screen h-16 sm:h-20 px-5 sm:px-8 sm:sticky sm:top-0 bg-black shadow",
            Row {
                class: "justify-center sm:justify-between w-full my-auto",
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
        Row {
            class: "w-screen shrink-0 h-16 sm:h-20 px-5 sm:px-8",
            Row {
                class: "w-full my-auto",
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
    let current_route: Route = use_route();
    let hidden = if is_navbar_hidden(&current_route) {
        "hidden"
    } else {
        "hidden sm:flex"
    };
    rsx! {
        Row {
            class: "{hidden} h-full rounded-full elevated elevated-border overflow-clip",
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
            class: "flex px-8 h-10 elevated-control",
            to: route,
            span {
                class: "text-sm font-medium my-auto font-semibold {color}",
                "{title}"
            }
        }
    }
}

fn MobileTabBar() -> Element {
    let current_route: Route = use_route();
    let hidden = if is_navbar_hidden(&current_route) {
        "hidden"
    } else {
        ""
    };
    rsx! {
        Row {
            class: "{hidden} sm:hidden fixed bottom-0 w-full elevated",
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
            Col {
                class: "mx-auto my-auto {color}",
                gap: 1,
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

fn is_navbar_hidden(current_route: &Route) -> bool {
    match current_route {
        Route::Swap {} | Route::Pay {} => true,
        _ => false,
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
            Route::Trade {} | Route::Market { market: _ } => true,
            _ => false,
        },
        _ => false,
    }
}
