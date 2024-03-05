use dioxus::prelude::*;

#[component]
pub fn HowItWorks(cx: Scope) -> Element {
    render! {
        div {
            h2 {
                "How it works"
            }
            p {
                "Coming soon..."
            }
        }
    }
}
