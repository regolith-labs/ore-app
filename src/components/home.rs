use dioxus::prelude::*;

use crate::components::{Activity, Balance};

pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            Balance {}
            Activity {}
        }
    }
}
