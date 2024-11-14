use dioxus::prelude::*;

use crate::components::{Breadcrumbs, Col};

#[component]
pub fn Pool(pool: String) -> Element {
    rsx! {
        Col {
            class: "w-full px-5 sm:px-8",
            gap: 4,
            Breadcrumbs {}
            span {
                class: "font-wide text-2xl font-semibold",
                "{pool}"
            }
        }
    }
}
