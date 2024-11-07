use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            "Home"
        }
    }
}
