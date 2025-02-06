use dioxus::prelude::*;

#[component]
pub fn NotFound(_route: Vec<String>) -> Element {
    rsx! {
        p {
            "Not found"
        }
    }
}
