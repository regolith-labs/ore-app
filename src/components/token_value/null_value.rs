use dioxus::prelude::*;

pub fn NullValue() -> Element {
    rsx! {
        span {
            class: "text-elements-midEmphasis font-medium",
            "–"
        }
    }
}
