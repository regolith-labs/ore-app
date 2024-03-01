use dioxus::prelude::*;

use crate::components::{Activity, Balance};

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            Balance {}
            Activity {}
        }
    }
}
