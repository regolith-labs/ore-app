use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use crate::{
    components::{ActivityIndicator, Spinner, StopButton},
    hooks::{
        use_gateway, use_miner_toolbar_state, use_power_level, use_priority_fee, use_proof,
        MinerStatusMessage, PowerLevel, PriorityFee, ReadMinerToolbarState,
        UpdateMinerToolbarState,
    },
    miner::{Miner, WEB_WORKERS},
};

#[component]
pub fn MinerToolbarActive(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();

    // Animate the hash in the miner toolbar to visualize mining.
    use_future(move || async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_millis(125)).await;
            if let MinerStatusMessage::Searching = toolbar_state.status_message() {
                toolbar_state.set_display_hash(Blake3Hash::new_unique());
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-row gap-2 max-w-screen w-screen justify-start my-auto px-4 sm:px-8 object-contain",
            div {
                class: "flex-shrink-0 flex-none my-auto",
                ActivityIndicator {}
            }
            p {
                class: "font-semibold text-white flex-shrink-0 flex-none my-auto",
                "Mining"
            }
            div {
                class: "flex-shrink flex-auto truncate my-auto",
                match toolbar_state.status_message() {
                    MinerStatusMessage::Searching => {
                        rsx! {
                            p {
                                class: "font-mono text-sm truncate flex-shrink flex-auto opacity-80 my-auto ml-2",
                                "{toolbar_state.display_hash()}"
                            }
                        }
                    }
                    MinerStatusMessage::Submitting => {
                        rsx! {
                            p {
                                class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                "Submitting hash..."
                            }
                        }
                    }
                    MinerStatusMessage::Error => {
                        rsx! {
                            p {
                                class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                "Error submitting transaction"
                            }
                        }
                    }
                }
            }
            div {
                class: "flex-shrink-0 flex-none ml-auto my-auto",
                StopButton {}
            }
        }
    }
}

pub fn PriorityFeeConfig() -> Element {
    let mut priority_fee = use_priority_fee();

    rsx! {
        div {
            class: "flex flex-row gap-8 justify-between mt-8",
            div {
                class: "flex flex-col gap-1",
                p {
                    class: "text-white font-semibold",
                    "Priority fee"
                }
                p {
                    class: "text-white text-xs opacity-80 max-w-96",
                    "Add a priority fee to increase your chances of landing a transaction."
                }
           }
           div {
                class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                input {
                    class: "bg-transparent text-white text-right px-1 mb-auto rounded font-semibold hover:bg-green-600 transition-colors",
                    dir: "rtl",
                    step: 100_000,
                    min: 0,
                    max: 10_000_000,
                    r#type: "number",
                    value: "{priority_fee.read().0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u64>() {
                            priority_fee.set(PriorityFee(v));
                        }
                    }
                }
                p {
                    class: "my-auto",
                    "microlamports"
                }
            }
        }
    }
}

fn DownloadLink() -> Element {
    // if cfg!(feature = "web") {
    //     rsx! {
    //         div {
    //             class: "flex flex-row gap-2 mt-8 p-2.5 rounded bg-green-600",
    //             WarningIcon {
    //                 class: "w-4 h-4 mt-0.5 shrink-0"
    //             }
    //             p {
    //                 class: "text-sm my-auto",
    //                 "You are mining from a web browser. For better performance, "
    //                 Link {
    //                     to: Route::Download {},
    //                     class: "font-medium underline",
    //                     "download the app."
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     None
    // }
    rsx! {}
}
