use dioxus::prelude::*;

use crate::components::Breadcrumbs;

#[component]
pub fn Pool(pool: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen px-5 sm:px-8",
            Breadcrumbs {}
            span {
                class: "font-wide text-2xl font-semibold",
                "{pool}"
            }
        }
    }
}
