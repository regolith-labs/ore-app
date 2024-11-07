use dioxus::prelude::*;

use crate::components::Navbar;

pub fn Trade() -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            Navbar {}
            "Trade"
        }
    }
}
