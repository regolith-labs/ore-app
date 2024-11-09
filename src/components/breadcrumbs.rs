use dioxus::prelude::*;

use crate::{components::CarrotRightIcon, route::Route};

pub fn Breadcrumbs() -> Element {
    let route: Route = use_route();
    match route {
        Route::Asset { asset } => {
            rsx! {
                div {
                    class: "flex flex-row gap-2 font-medium text-xs",
                    Link {
                        to: Route::Trade {},
                        class: "text-gray-700 py-2",
                        "Trade"
                    }
                    CarrotRightIcon {
                        class: "w-3 text-gray-700",
                    }
                    span {
                        class: "py-2",
                        "{asset}"
                    }
                }
            }
        }
        _ => rsx! {},
    }
}
