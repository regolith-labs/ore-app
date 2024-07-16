use dioxus::prelude::*;

use crate::{
    components::{StartButton, StopButton},
    hooks::{use_miner_toolbar_state, ReadMinerToolbarState},
};

pub fn MinerToolbarError() -> Element {
    let miner_state = use_miner_toolbar_state();
    if miner_state.is_open() {
        rsx! {
            div {
                class: "flex flex-col grow w-full gap-4 px-4 py-6 sm:px-8",
                div {
                    class: "flex flex-col w-full gap-2",
                    div {
                        class: "flex flex-row w-full justify-between",
                        h2 {
                            class: "text-3xl md:text-4xl lg:text-5xl text-white font-bold",
                            "Error"
                        }
                        div {
                            class: "my-auto",
                            StopButton {}
                        }
                    }
                    p {
                        class: "text-lg text-white",
                        "RPC rate limited."
                    }
                    p {
                        class: "font-mono text-sm truncate shrink opacity-80",
                        "Please try again in a moment."
                    }
                }
            }
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
}
