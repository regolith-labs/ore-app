use dioxus::prelude::*;

use crate::{components::CarrotRightIcon, route::Route};

pub fn Breadcrumbs() -> Element {
    let route: Route = use_route();
    rsx! {
        div {
            class: "flex flex-row gap-1 font-medium text-xs sm:text-sm",
            match route {
                Route::Pair { pair } => {
                    rsx! {
                        Path {
                            title: "Stake",
                            route: Route::Stake {},
                        }
                        Active {
                            title: "{pair}"
                        }
                    }
                }
                Route::Pool { pool } => {
                    rsx! {
                        Path {
                            title: "Mine",
                            route: Route::Mine {},
                        }
                        Active {
                            title: "{pool}"
                        }
                    }
                }
                Route::Asset { asset } => {
                    rsx! {
                        Path {
                            title: "Trade",
                            route: Route::Trade {},
                        }
                        Active {
                            title: "{asset}"
                        }
                    }
                }
                _ => rsx! {},
            }
        }
    }
}

#[component]
fn Path(title: String, route: Route) -> Element {
    rsx! {
        span {
            class: "flex flex-row gap-1",
            Link {
                to: route,
                class: "text-gray-700 hover:underline",
                "{title}"
            }
            CarrotRightIcon {
                class: "w-3 text-gray-700",
            }
        }
    }
}

#[component]
fn Active(title: String) -> Element {
    rsx! {
        span {
            class: "text-white",
            "{title}"
        }
    }
}
