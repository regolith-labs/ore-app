use dioxus::prelude::*;

use crate::{
    components::{
        ActivityIndicator, IsToolbarOpen, MinerDisplayHash, MinerPower, OreIcon, Spinner,
        StopButton, Tooltip, TooltipDirection,
    },
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
    let status_message = *use_shared_state::<MinerStatusMessage>(cx).unwrap().read();
    let display_hash = use_shared_state::<MinerDisplayHash>(cx)
        .unwrap()
        .read()
        .0
        .to_string();

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
                        match status_message {
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
                        }
                    }
                    p {
                        class: "font-mono text-sm truncate opacity-60",
                        "{display_hash}"
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
                    match status_message {
                        MinerStatusMessage::Searching => {
                            render! {
                                p {
                                    class: "font-mono text-sm truncate opacity-80 my-auto ml-2",
                                    "{display_hash}"
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
