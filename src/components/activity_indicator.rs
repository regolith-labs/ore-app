use dioxus::prelude::*;

#[allow(dead_code)]
pub fn ActivityIndicator() -> Element {
    rsx! {
        span {
            class: "relative flex h-3 w-3 justify center my-auto",
            span {
                class: "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 bg-white",
                " "
            }
            span {
                class: "relative inline-flex rounded-full h-2 w-2 my-auto mx-auto bg-white"
            }
        }
    }
}
