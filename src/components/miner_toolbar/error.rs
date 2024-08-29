use dioxus::prelude::*;

use crate::{
    components::StartButton,
    hooks::{use_miner_toolbar_state, MinerStatusMessage, ReadMinerToolbarState},
};

pub fn MinerToolbarError() -> Element {
    let toolbar_state = use_miner_toolbar_state();
    rsx! {
        div {
            class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
            p {
                class: "font-semibold text-white flex-shrink-0 flex-none my-auto",
                "Error"
            }
            div {
                class: "flex-shrink flex-auto truncate my-auto",
                // p {
                //     class: "font-mono text-sm truncate flex-shrink flex-auto opacity-80 my-auto ml-2",
                //     "RPC rate limited."
                // }
                match toolbar_state.status_message() {
                    MinerStatusMessage::Error => {
                        rsx! {
                            p {
                                class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                "Error submitting transaction"
                            }
                        }
                    }
                    MinerStatusMessage::SignatureDenied => {
                        rsx! {
                            p {
                                class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                "Signature denied"
                            }
                        }
                    }
                    _ => rsx! {}
                }
            }
            div {
                class: "flex-shrink-0 flex-none ml-auto my-auto",
                StartButton {}
            }
        }
    }
}
