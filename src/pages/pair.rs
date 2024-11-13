use dioxus::prelude::*;

use crate::components::Breadcrumbs;

#[component]
pub fn Pair(pair: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full px-5 sm:px-8",
            Breadcrumbs {}
            span {
                class: "font-wide text-2xl font-semibold",
                "{pair}"
            }
        }
    }
}
