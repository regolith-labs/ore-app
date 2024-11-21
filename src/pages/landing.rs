use dioxus::prelude::*;

pub fn Landing() -> Element {
    rsx! {
        div {
            class: "flex h-screen w-screen p-8 pb-20 sm:pb-16",
            span {
                class: "mx-auto my-auto font-wide text-xl sm:text-2xl font-medium",
                "Coming soon..."
            }
        }
    }
}
