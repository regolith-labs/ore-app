use dioxus::prelude::*;

use crate::route::Route;

pub fn Landing() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full",
            Link {
                class: "mx-auto my-auto",
                to: Route::Mine {},
                "Launch app"
            }
        }
    }
}
