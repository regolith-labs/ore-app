use dioxus::prelude::*;
use ore::state::Treasury;
#[cfg(feature = "desktop")]
use solana_account_decoder::parse_token::UiTokenAmount;
#[cfg(feature = "web")]
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::{
    components::{
        ActivityIndicator, IsToolbarOpen, MinerDisplayHash, MinerPower, OreIcon, Spinner,
        StopButton, Tooltip, TooltipDirection,
    },
    gateway::AsyncResult,
    miner::Miner,
};

use super::MinerStatusMessage;

#[derive(Props, PartialEq)]
pub struct MinerToolbarActiveProps {
    pub treasury: AsyncResult<Treasury>,
    pub ore_supply: AsyncResult<UiTokenAmount>,
    pub miner: UseState<Miner>,
}

#[component]
pub fn MinerToolbarActive(cx: Scope<MinerToolbarActiveProps>) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let status_message = use_shared_state::<MinerStatusMessage>(cx)
        .unwrap()
        .read()
        .0
        .to_string();
    let display_hash = use_shared_state::<MinerDisplayHash>(cx)
        .unwrap()
        .read()
        .0
        .to_string();

    // let circulating_supply = match cx.props.treasury {
    //     AsyncResult::Ok(treasury) => {
    //         (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
    //     }
    //     _ => 0f64,
    // };

    // let ore_supply = match cx.props.ore_supply.clone() {
    //     AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap().to_string(),
    //     AsyncResult::Loading => "-".to_string(),
    //     AsyncResult::Error(_err) => "Err".to_string(),
    // };

    let reward_rate = match &cx.props.treasury {
        AsyncResult::Ok(treasury) => {
            (treasury.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col grow w-full gap-1 px-4 py-6 sm:px-8 sm:py-8",
                div {
                    class: "flex flex-row w-full justify-between",
                    h2 {
                        class: "text-3xl md:text-4xl lg:text-5xl text-white font-bold",
                        "Mining"
                    }
                    div {
                        class: "flex flex-row gap-4",
                        StopButton {
                            miner: cx.props.miner.clone()
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-2 sm:gap-3 w-full",
                    div {
                        class: "flex flex-row gap-4",
                        p {
                            class: "text-lg text-white",
                            "{status_message}"
                        }
                        if status_message.eq("Submitting hash for validation..." ) {
                            render! {
                                Spinner {
                                    class: "my-auto"
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
                    p {
                        class: "text-sm text-white opacity-80 my-auto ml-2",
                        "{status_message}"
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
