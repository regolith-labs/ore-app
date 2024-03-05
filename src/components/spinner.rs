use dioxus::prelude::*;

#[component]
pub fn Spinner<'a>(cx: Scope, class: Option<&'a str>) -> Element {
    let class = class.unwrap_or("");
    render! {
        div {
            class: "{class} w-6 h-6 border-4 rounded-full border-t-white border-white/30 animate-spin"
        }
    }
}
