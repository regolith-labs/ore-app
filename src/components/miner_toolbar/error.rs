use dioxus::prelude::*;

use crate::{
    components::{MinerToolbarInsufficientFunds, StartButton},
    hooks::{use_miner_toolbar_state, ReadMinerToolbarState},
};

pub fn MinerToolbarError() -> Element {
    let miner_state = use_miner_toolbar_state();
    if miner_state.is_open() {
        rsx! {
            MinerToolbarInsufficientFunds {}
        }
    } else {
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
