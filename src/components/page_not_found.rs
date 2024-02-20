use dioxus::prelude::*;

#[component]
pub fn PageNotFound(cx: Scope, _route: Vec<String>) -> Element {
    render! {
        p {
            "Page not found"
        }
    }
}
