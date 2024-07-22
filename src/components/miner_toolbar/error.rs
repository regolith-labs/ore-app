use dioxus::prelude::*;

use crate::components::StartButton;

pub fn MinerToolbarError() -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
            p {
                class: "font-semibold text-white flex-shrink-0 flex-none my-auto",
                "Error"
            }
            div {
                class: "flex-shrink flex-auto truncate my-auto",
                p {
                    class: "font-mono text-sm truncate flex-shrink flex-auto opacity-80 my-auto ml-2",
                    "RPC rate limited."
                }
            }
            div {
                class: "flex-shrink-0 flex-none ml-auto my-auto",
                StartButton {}
            }
        }
    }
}
