use dioxus::prelude::*;

#[component]
pub fn Spinner(cx: Scope) -> Element {
    render! {
        div {
            class: "mx-auto w-6 h-6 border-4 rounded-full border-t-white border-white/30 animate-spin"
        }
    }
}
