use dioxus::prelude::*;

#[component]
pub fn PageNotFound(_route: Vec<String>) -> Element {
    rsx! {
        p {
            "Page not found"
        }
    }
}
