use dioxus::prelude::*;

#[component]
pub fn Tutorial(cx: Scope) -> Element {
    render! {
        div {
            class: "absolute right-8 bottom-20 px-3 py-2 animate-bounce bg-green-500 text-white rounded shadow-sm",
            p {
                class: "font-medium",
                "Click here to start mining."
            }
        }
    }
}
