use dioxus::prelude::*;

#[component]
pub fn Tokenomics(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4",
            h2 {
                "Ore tokenomics"
            }
            p {
                "Coming soon..."
            }
        }
    }
}
