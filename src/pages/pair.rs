use dioxus::prelude::*;

#[component]
pub fn Pair(lp_mint: String) -> Element {
    rsx! {
        "{lp_mint}"
    }
}
