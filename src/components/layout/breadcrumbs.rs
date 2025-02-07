use dioxus::prelude::*;

use crate::{
    components::{CarrotRightIcon, Row},
    route::Route,
};

pub fn _Breadcrumbs() -> Element {
    let route: Route = use_route();
    rsx! {
        Row {
            class: "font-medium text-xs sm:text-sm",
            gap: 1,
            match route {
                // Route::Pair { pair } => {
                //     rsx! {
                //         Path {
                //             title: "Stake",
                //             route: Route::Stake {},
                //         }
                //         Active {
                //             title: "{pair}"
                //         }
                //     }
                // }
                // Route::Pool { pool } => {
                //     rsx! {
                //         Path {
                //             title: "Mine",
                //             route: Route::Mine {},
                //         }
                //         Active {
                //             title: "{pool}"
                //         }
                //     }
                // }
                // Route::Market { market } => {
                //     rsx! {
                //         Path {
                //             title: "Trade",
                //             route: Route::Trade {},
                //         }
                //         Active {
                //             title: "{market}"
                //         }
                //     }
                // }
                _ => rsx! {},
            }
        }
    }
}

#[component]
fn Path(title: String, route: Route) -> Element {
    rsx! {
        Row {
            gap: 1,
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
