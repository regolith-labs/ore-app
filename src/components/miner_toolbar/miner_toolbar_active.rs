use dioxus::prelude::*;
use ore::state::{Proof, Treasury};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::{
    components::{
        stop_mining, IsModalOpen, IsToolbarOpen, MinerStatus, OreIcon, PauseIcon, Tooltip,
        TooltipDirection,
    },
    gateway::AsyncResult,
};

// TODO Lifetime rewards
// TODO Lifetime hashes

pub enum MinerChart {
    Hash,
    Time,
    Rewards,
    Rate,
    Circulating,
    Supply,
}

#[derive(Props, PartialEq)]
pub struct MinerToolbarActiveProps {
    pub treasury: AsyncResult<Treasury>,
    pub proof: AsyncResult<Proof>,
    pub ore_supply: AsyncResult<UiTokenAmount>,
}

#[component]
pub fn MinerToolbarActive(cx: Scope<MinerToolbarActiveProps>) -> Element {
    let chart = use_state(cx, || MinerChart::Hash);
    let timer = use_state(cx, || 0u64);
    let proof = cx.props.proof;
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();

    let hash = match proof {
        AsyncResult::Ok(proof) => proof.hash.to_string(),
        _ => "–".to_string(),
    };

    let hash_abbr = if hash.len().gt(&16) {
        hash[0..16].to_string()
    } else {
        "–".to_string()
    };

    let claimable_rewards = match cx.props.proof {
        AsyncResult::Ok(proof) => {
            (proof.claimable_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    let circulating_supply = match cx.props.treasury {
        AsyncResult::Ok(treasury) => {
            (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    let ore_supply = match cx.props.ore_supply.clone() {
        AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap().to_string(),
        AsyncResult::Loading => "-".to_string(),
        AsyncResult::Error(_err) => "Err".to_string(),
    };

    let reward_rate = match &cx.props.treasury {
        AsyncResult::Ok(treasury) => {
            (treasury.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };

    use_effect(cx, &proof, |_| {
        timer.set(0);
        async move {}
    });

    let _n = use_future(cx, (), |_| {
        let timer = timer.clone();
        async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                timer.set(*timer.current() + 1);
            }
        }
    });

    // let container_class =
    //     "flex flex-col gap-1 justify-between px-3 py-2 rounded hover:bg-green-600 active:bg-green-700 cursor-pointer transition transition-colors";
    let container_class = "flex flex-col gap-1 shrink h-min";
    let header_container_class = "flex flex-row justify-start gap-1.5";
    let header_class = "font-medium text-xs z-0 text-nowrap opacity-80";
    let mono_value_class = "font-mono text-white";
    let value_class = "font-medium text-white";

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col flex-grow justify-start gap-8 px-4 sm:px-8 py-8",
                div {
                    class: "flex flex-row w-full justify-between",
                    h2 {
                        class: "text-2xl text-white font-bold",
                        "Mining"
                    }
                    div {
                        class: "flex flex-row gap-4",
                        ClaimButton {
                            claimable_rewards: claimable_rewards
                        }
                        StopButton {}
                    }
                }
                div {
                    class: "grid grid-cols-2 grid-rows-3 gap-y-8",
                    div {
                        class: "{container_class} w-full md:w-3/4",
                        onclick: |_| {
                            chart.set(MinerChart::Hash);
                        },
                        div {
                            class: "{header_container_class}",
                            p {
                                class: "{header_class}",
                                "Hash"
                            }
                            Tooltip {
                                text: "The calculation your miner is currently working on.",
                                direction: TooltipDirection::Right
                            }
                        }
                        p {
                            class: "{mono_value_class} hidden lg:block",
                            "{hash}"
                        }
                        p {
                            class: "{mono_value_class} lg:hidden",
                            "{hash_abbr}"
                        }
                    }
                    div {
                        class: "{container_class} w-full md:w-1/4",
                        onclick: |_| {
                            chart.set(MinerChart::Time);
                        },
                        div {
                            class: "{header_container_class}",
                            p {
                                class: "{header_class}",
                                "Time"
                            }
                            Tooltip {
                                text: "The time your miner has spent on the current hash.",
                                direction: TooltipDirection::Right
                            }
                        }
                        p {
                            class: "{value_class}",
                            "{timer.current()} sec"
                        }
                    }
                    div {
                        class: "{container_class} w-full",
                        onclick: |_| {
                            chart.set(MinerChart::Rewards);
                        },
                        div {
                            class: "{header_container_class}",
                            p {
                                class: "{header_class}",
                                "Rewards"
                            }
                            Tooltip {
                                text: "The amount of Ore you have mined and may claim.",
                                direction: TooltipDirection::Right
                            }
                        }
                        p {
                            class: "{value_class} flex flex-row flex-nowrap text-nowrap place-items-baseline",
                            OreIcon {
                                class: "w-4 h-4 my-auto",
                            }
                            span {
                                class: "ml-1.5",
                                "{claimable_rewards}"
                            }
                        }
                    }
                    div {
                        class: "{container_class} w-full",
                        onclick: |_| {
                            chart.set(MinerChart::Rate);
                        },
                        div {
                            class: "{header_container_class}",
                            p {
                                class: "{header_class}",
                                "Rate"
                            }
                            Tooltip {
                                text: "The amount of Ore you are earning per hash.",
                                direction: TooltipDirection::Right
                            }
                        }
                        p {
                            class: "{value_class} flex flex-row flex-nowrap text-nowrap place-items-baseline",
                            OreIcon {
                                class: "w-4 h-4 my-auto",
                            }
                            span {
                                class: "ml-1.5",
                                "{reward_rate}"
                            }
                        }
                    }
                    div {
                        class: "{container_class} w-full",
                        onclick: |_| {
                            chart.set(MinerChart::Circulating);
                        },
                        div {
                            class: "{header_container_class}",
                            p {
                                class: "{header_class}",
                                "Circulating"
                            }
                            Tooltip {
                                text: "The total amount of Ore that has ever been claimed.",
                                direction: TooltipDirection::Right
                            }
                        }
                        p {
                            class: "{value_class} flex flex-row flex-nowrap text-nowrap place-items-baseline",
                            OreIcon {
                                class: "w-4 h-4 my-auto",
                            }
                            span {
                                class: "ml-1.5",
                                "{circulating_supply}"
                            }
                        }
                    }
                    div {
                        class: "{container_class} w-full",
                        onclick: |_| {
                            chart.set(MinerChart::Supply);
                        },
                        div {
                            class: "{header_container_class}",
                            p {
                                class: "{header_class}",
                                "Supply"
                            }
                            Tooltip {
                                text: "The total amount of Ore that has ever been mined.",
                                direction: TooltipDirection::Right
                            }
                        }
                        p {
                            class: "{value_class} flex flex-row flex-nowrap text-nowrap place-items-baseline",
                            OreIcon {
                                class: "w-4 h-4 my-auto",
                            }
                            span {
                                class: "ml-1.5",
                                "{ore_supply}"
                            }
                        }
                    }
                }
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
                RewardsCounter {
                    claimable_rewards: claimable_rewards,
                }
                div {
                    class: "flex flex-row gap-2 sm:gap-4",
                    ClaimButton {
                        claimable_rewards: claimable_rewards
                    }
                    StopButton {}
                }
            }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct ClaimButtonProps {
    pub claimable_rewards: f64,
}

#[component]
pub fn ClaimButton(cx: Scope<ClaimButtonProps>) -> Element {
    let is_claim_modal_open = use_shared_state::<IsModalOpen>(cx).unwrap();
    let opacity = if cx.props.claimable_rewards.gt(&0f64) {
        "opacity-100"
    } else {
        "opacity-0 pointer-events-none"
    };
    render! {
        button {
            class: "transition transition-colors text-sm font-semibold px-4 h-10 hover:bg-green-600 active:bg-green-700 rounded-full {opacity}",
            onclick: |_| {
                *is_claim_modal_open.write() = IsModalOpen(true);
            },
            "Claim"
        }
    }
}

#[derive(Props, PartialEq)]
pub struct RewardsCounterProps {
    pub claimable_rewards: f64,
}

#[component]
pub fn RewardsCounter(cx: Scope<RewardsCounterProps>) -> Element {
    let claimable_rewards = cx.props.claimable_rewards;

    if claimable_rewards.gt(&0f64) {
        render! {
            div {
                class: "flex flex-row gap-3 my-auto",
                ActivityIndicator {}
                div {
                    class: "flex flex-row gap-1 my-auto text-white",
                    OreIcon {
                        class: "h-3.5 w-3.5 my-auto",
                    }
                    p {
                        class: "font-medium",
                        "{claimable_rewards}"
                    }
                }
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row gap-2 my-auto",
                ActivityIndicator {}
                p {
                    class: "font-medium text-white",
                    "Mining"
                }
            }
        }
    }
}

#[component]
pub fn ActivityIndicator(cx: Scope) -> Element {
    // let class = cx.props.class.unwrap_or("");
    render! {
        span {
            class: "relative flex h-3 w-3 justify center my-auto",
            span {
                class: "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 bg-white",
                " "
            }
            span {
                class: "relative inline-flex rounded-full h-2 w-2 my-auto mx-auto bg-white"
            }
        }
    }
}

#[component]
pub fn StopButton(cx: Scope) -> Element {
    let status = use_shared_state::<MinerStatus>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    render! {
        button {
            class: "transition transition-colors flex w-10 h-10 justify-center rounded-full hover:bg-green-600 active:bg-green-700",
            onclick: move |_e| {
                // let worker = worker.clone();
                let status = status.clone();
                let is_toolbar_open = is_toolbar_open.clone();
                async move {
                    // stop_mining(&status, &is_toolbar_open, worker);
                    stop_mining(&status, &is_toolbar_open);
                }
            },
            PauseIcon {
                class: "w-6 h-6 my-auto"
            }
        }
    }
}
