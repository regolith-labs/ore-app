use dioxus::prelude::*;

use crate::route::Route;

pub fn Landing() -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            "Landing"
            Link {
                to: Route::Mine {},
                "Launch app"
            }
        }
    }
}
