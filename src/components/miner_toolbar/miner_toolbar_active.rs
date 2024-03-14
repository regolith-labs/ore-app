use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
#[cfg(feature = "desktop")]
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    components::{
        ActivityIndicator, IsToolbarOpen, MinerDisplayHash, MinerPower, OreIcon, Spinner,
        StopButton, Tooltip, TooltipDirection,
    },
    hooks::{use_priority_fee, PriorityFee},
    miner::Miner,
};

use super::MinerStatusMessage;

#[derive(Props, PartialEq)]
pub struct MinerToolbarActiveProps {
    pub miner: UseState<Miner>,
}

#[component]
pub fn MinerToolbarActive(cx: Scope<MinerToolbarActiveProps>) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let miner_status_message = *use_shared_state::<MinerStatusMessage>(cx).unwrap().read();
    let miner_display_hash = use_shared_state::<MinerDisplayHash>(cx)
        .unwrap()
        .read()
        .0
        .to_string();
    let priority_fee = use_priority_fee(cx);

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col grow w-full gap-4 px-4 py-6 sm:px-8",
                div {
                    class: "flex flex-row w-full justify-between",
                    h2 {
                        class: "text-3xl md:text-4xl lg:text-5xl text-white font-bold",
                        "Mining"
                    }
                    div {
                        class: "my-auto",
                        StopButton {
                            miner: cx.props.miner.clone()
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-2 sm:gap-3 w-full",
                    div {
                        class: "flex flex-row gap-4",
                        match miner_status_message {
                            MinerStatusMessage::Searching => {
                                render! {
                                    p {
                                        class: "text-lg text-white",
                                        "Searching for a valid hash..."
                                    }
                                }
                            }
                            MinerStatusMessage::Submitting => {
                                render! {
                                    p {
                                        class: "text-lg text-white",
                                        "Submitting hash for validation..."
                                    }
                                    Spinner {
                                        class: "my-auto"
                                    }
                                }
                            }
                            MinerStatusMessage::Error => {
                                render! {
                                    p {
                                        class: "text-lg text-white",
                                        "Error submit transaction"
                                    }
                                }
                            }
                            _ => None
                        }
                    }
                    p {
                        class: "font-mono text-sm truncate opacity-80",
                        "{miner_display_hash}"
                    }
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
                                "When Solana is busy, priority fees can increase the chances of your transactions being accepted."
                            }

                        }
                        div {
                            class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                            input {
                                class: "bg-transparent text-white text-right px-1 mb-auto",
                                step: 1,
                                min: 0,
                                max: LAMPORTS_PER_SOL as i64,
                                r#type: "number",
                                value: "{priority_fee.read().0}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value.parse::<u64>() {
                                        *priority_fee.write() = PriorityFee(v);
                                    }
                                }
                            }
                            p {
                                class: "my-auto",
                                "lamports"
                            }
                        }
                    }
                }
                div {
                    class: "mt-16",
                    MinerPower {}
                }
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
                div {
                    class: "flex flex-row gap-2 my-auto",
                    ActivityIndicator {}
                    p {
                        class: "font-semibold text-white",
                        "Mining"
                    }
                    match miner_status_message {
                        MinerStatusMessage::Searching => {
                            render! {
                                p {
                                    class: "font-mono text-sm truncate opacity-80 my-auto ml-2",
                                    "{miner_display_hash}"
                                }
                            }
                        }
                        MinerStatusMessage::Submitting => {
                            render! {
                                p {
                                    class: "text-sm text-white opacity-80 my-auto ml-2",
                                    "Submitting hash for validation..."
                                }
                            }
                        }
                        MinerStatusMessage::Error => {
                            render! {
                                p {
                                    class: "text-sm text-white opacity-80 my-auto ml-2",
                                    "Error submitting transaction"
                                }
                            }
                        }
                        _ => None
                    }
                }
                StopButton {
                    miner: cx.props.miner.clone()
                }
            }
        }
    }
}

#[component]
pub fn MinerDataOre<'a>(cx: Scope, title: &'a str, tooltip: &'a str, amount: String) -> Element {
    let container_class = "flex flex-col gap-0 shrink h-min";
    let header_container_class = "flex flex-row justify-start gap-1.5";
    let header_class = "font-medium text-xs z-0 text-nowrap opacity-80";
    let value_class = "font-medium text-white h-8";
    render! {
        div {
            class: "{container_class} w-full",
            div {
                class: "{header_container_class}",
                p {
                    class: "{header_class}",
                    "{title}"
                }
                Tooltip {
                    text: "{tooltip}",
                    direction: TooltipDirection::Right
                }
            }
            div {
                class: "flex flex-row gap-8",
                p {
                    class: "{value_class} flex flex-row flex-nowrap text-nowrap place-items-baseline",
                    OreIcon {
                        class: "w-4 h-4 my-auto",
                    }
                    span {
                        class: "ml-1.5 my-auto",
                        "{amount}"
                    }
                }
            }
        }
    }
}
