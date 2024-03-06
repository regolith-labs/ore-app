use dioxus::prelude::*;

#[component]
pub fn OreEconomics(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4",
            h2 {
                "Ore economics"
            }
            p {
                "Coming soon..."
            }
        }
    }
}
