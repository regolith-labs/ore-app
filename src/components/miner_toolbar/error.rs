use dioxus::prelude::*;

use crate::components::{IsToolbarOpen, MinerToolbarInsufficientFunds, StartButton};

#[component]
pub fn MinerToolbarError(cx: Scope) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    if is_toolbar_open.read().0 {
        render! {
            MinerToolbarInsufficientFunds {}
        }
    } else {
        render! {
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
                        "Transaction failed."
                    }
                }
                div {
                    class: "flex-shrink-0 flex-none ml-auto my-auto",
                    StartButton {}
                }
            }
        }
    }
}
