use dioxus::prelude::*;

pub fn LoadingValue() -> Element {
    rsx! {
        span {
            class: "w-10 h-4 rounded my-auto loading",
            ""
        }
    }
}
