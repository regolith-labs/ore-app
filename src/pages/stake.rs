use dioxus::prelude::*;

pub fn Stake() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen px-5 sm:px-8",
            span {
                class: "font-wide text-2xl font-semibold",
                "Liquidity"
            }
        }
    }
}
