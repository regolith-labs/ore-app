use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{Activity, Balance, OreIcon, Spinner},
    hooks::{
        use_gateway, use_miner_toolbar_state, use_power_level, use_proof, MinerStatus,
        MinerStatusMessage, PowerLevel, ReadMinerToolbarState,
    },
    miner::WEB_WORKERS,
};

// TODO Activity history of hashes
// TODO Display for non-active states
// TODO Stop start button

pub fn Mine() -> Element {
    let toolbar_state = use_miner_toolbar_state();
    rsx! {
        div {
            class: "flex flex-col gap-8 overflow-visible",
            div {
                class: "flex flex-col gap-3",
                h2 {
                    "Miner"
                }
                match toolbar_state.status() {
                    MinerStatus::NotStarted => {
                        rsx! {
                            p {
                                class: "text-lg text-white",
                                "Stopped"
                            }
                        }
                    },
                    MinerStatus::Activating => {
                        rsx! {
                            p {
                                class: "text-lg text-white",
                                "Starting..."
                            }
                        }
                    },
                    MinerStatus::Error => {
                        rsx! {
                            p {
                                class: "text-lg text-white",
                                "Error"
                            }
                        }
                    },
                    MinerStatus::Active => {
                        rsx! {
                            match toolbar_state.status_message() {
                                MinerStatusMessage::Searching => {
                                    rsx! {
                                        p {
                                            class: "text-lg text-white",
                                            "Searching for a valid hash... "
                                            // if time_remaining.read().gt(&0) {
                                            //     "({time_remaining} sec)"
                                            // }
                                        }
                                    }
                                }
                                MinerStatusMessage::Submitting => {
                                    rsx! {
                                        div {
                                            class: "flex flex-row gap-2",
                                            p {
                                                class: "text-lg text-white",
                                                "Submitting hash for validation..."
                                            }
                                            Spinner {
                                                class: "my-auto"
                                            }
                                        }
                                    }
                                }
                                MinerStatusMessage::Error => {
                                    rsx! {
                                        p {
                                            class: "text-lg text-white",
                                            "Error submitting transaction"
                                        }
                                    }
                                }
                            }
                            match toolbar_state.status_message() {
                                MinerStatusMessage::Searching | MinerStatusMessage::Submitting => {
                                    rsx! {
                                        p {
                                            class: "font-mono text-sm truncate shrink opacity-80",
                                            "{toolbar_state.display_hash()}"
                                        }
                                    }
                                }
                                _ => rsx! {}
                            }
                        }
                    },
                }
            }
            StakeBalanceDisplay {}
            MultiplierDisplay {}
            PowerLevelConfig {}
        }
    }
}

pub fn StakeBalanceDisplay() -> Element {
    let proof = use_proof();

    rsx! {
        div {
            class: "flex flex-col gap-2",
            div {
                class: "flex flex-row gap-8 justify-between",
                    p {
                        class: "text-white font-semibold",
                        "Stake"
                    }
               div {
                    class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                    if let Some(proof) = *proof.read() {
                        if let Ok(proof) = proof {
                            div {
                                class: "flex flex-row gap-2",
                                OreIcon {
                                    class: "my-auto w-4 h-4"
                                }
                                p {
                                    class: "font-semibold",
                                    "{amount_to_ui_amount(proof.balance, ore_api::consts::TOKEN_DECIMALS)}"
                                }
                            }
                        } else {
                            div {
                                class: "flex flex-row gap-2",
                                OreIcon {
                                    class: "my-auto w-4 h-4"
                                }
                                p {
                                    class: "font-semibold",
                                    "0"
                                }
                            }
                        }
                    } else {
                        div {
                            class: "flex flex-row w-32 h-10 grow loading rounded",
                        }
                    }
                }
            }
            p {
                class: "text-white text-xs opacity-80 max-w-96",
                "The amount of ORE you have staked with your miner."
            }
        }
    }
}

pub fn MultiplierDisplay() -> Element {
    let proof = use_proof();

    let multiplier = use_resource(move || async move {
        let gateway = use_gateway();
        if let Some(Ok(proof)) = *proof.read() {
            if let Ok(config) = gateway.get_config().await {
                return 1.0 + (proof.balance as f64 / config.max_stake as f64).min(1.0f64);
            }
        }
        1.0
    });

    rsx! {
        div {
            class: "flex flex-col gap-2",
            div {
                class: "flex flex-row gap-8 justify-between",
                    p {
                        class: "text-white font-semibold",
                        "Multiplier"
                    }
               div {
                    class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                    p {
                        class: "text-white text-right px-1 mb-auto font-semibold",
                        "{multiplier.read().unwrap_or(1.0):.12}x"
                    }
                }
            }
            p {
                class: "text-white text-xs opacity-80 max-w-96",
                "The multiplier you are earning from your mining rewards from staking."
            }
        }
    }
}

pub fn PowerLevelConfig() -> Element {
    let mut power_level = use_power_level();
    let max = *WEB_WORKERS as i64;

    rsx! {
        div {
            class: "flex flex-col gap-2",
            div {
                class: "flex flex-row gap-8 justify-between",
                    p {
                        class: "text-white font-semibold",
                        "Power level"
                    }
                div {
                    class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                    input {
                        class: "bg-transparent text-white text-right px-1 mb-auto rounded font-semibold transition-colors",
                        dir: "rtl",
                        step: 1,
                        min: 1,
                        max: max,
                        r#type: "number",
                        value: "{power_level.read().0}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u64>() {
                                power_level.set(PowerLevel(v));
                            }
                        }
                    }
                    p {
                        class: "my-auto",
                        "of {max} cores"
                    }
                }
            }
            p {
                class: "text-white text-xs opacity-80 max-w-96",
                "The number of computer cores you have dedicated to mining."
            }
        }
    }
}
