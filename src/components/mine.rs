use std::str::FromStr;

use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{BackButton, CreateAccountPage, MigrateAccountPage, OreIcon, Spinner},
    hooks::{
        use_escrow, use_gateway, use_miner_toolbar_state, use_power_level, use_proof, MinerStatus,
        MinerStatusMessage, PowerLevel, ReadMinerToolbarState,
    },
    miner::WEB_WORKERS,
};

// TODO Activity history of hashes
// TODO Display for non-active states
// TODO Stop start button

pub fn Mine() -> Element {
    let nav = use_navigator();
    let escrow = use_escrow();
    let proof = use_proof();

    if let Some(Ok(_escrow)) = *escrow.read() {
        return rsx! {
            MigrateAccountPage {
                escrow: escrow,
                proof: proof
            }
        };
    }

    if let Some(proof_result) = *proof.read() {
        if proof_result.is_err() {
            return rsx! {
                CreateAccountPage {}
            };
        }
    }

    rsx! {
        div {
            class: "flex flex-col gap-8 overflow-visible",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        "Miner"
                    }
                    MinerStatus {}
                }
            }
            div {
                class: "flex flex-col gap-6",
                StakeBalanceDisplay {}
                MultiplierDisplay {}
                PowerLevelConfig {}
            }
            // BoostConfig {}
        }
    }
}

pub fn MinerStatus() -> Element {
    let toolbar_state = use_miner_toolbar_state();
    rsx! {
        match toolbar_state.status() {
            MinerStatus::Active => {
                rsx! {
                    match toolbar_state.status_message() {
                        MinerStatusMessage::Searching => {
                            rsx! {
                                p {
                                    class: "text-lg text-white",
                                    "Searching for valid hashes... "
                                    // if time_remaining.read().gt(&0) {
                                    //     "({time_remaining} sec)"
                                    // }
                                }
                            }
                        }
                        MinerStatusMessage::Submitting(attempt) => {
                            rsx! {
                                div {
                                    class: "flex flex-row gap-2",
                                    p {
                                        class: "text-lg text-white",
                                        if attempt.eq(&0) {
                                            "Signature needed"
                                        } else {
                                            "Submitting transaction... (attempt {attempt})"
                                        }
                                        // "Submitting best hash..."
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
                        MinerStatusMessage::SignatureDenied => {
                            rsx! {
                                p {
                                    class: "text-lg text-white",
                                    "Signature denied"
                                }
                            }
                        }
                    }
                    match toolbar_state.status_message() {
                        MinerStatusMessage::Searching | MinerStatusMessage::Submitting(_) => {
                            rsx! {
                                p {
                                    class: "font-mono text-sm truncate shrink text-gray-300",
                                    "{toolbar_state.display_hash()}"
                                }
                            }
                        }
                        _ => rsx! {}
                    }
                }
            }
            _ => { rsx! {} },
        }
    }
}

pub fn StakeBalanceDisplay() -> Element {
    let proof = use_proof();

    rsx! {
            div {
                class: "flex flex-row gap-8 justify-between",
                    p {
                        class: "text-gray-300 font-medium text-sm my-auto",
                        "Unclaimed"
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
                            class: "flex flex-row w-32 h-8 grow loading rounded",
                        }
                    }
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
                return 1.0 + (proof.balance as f64 / config.top_balance as f64).min(1.0f64);
            }
        }
        1.0
    });

    rsx! {
        div {
            class: "flex flex-row gap-8 justify-between",
                p {
                    class: "text-gray-300 font-medium text-sm my-auto",
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
    }
}

#[allow(dead_code)]
pub fn BoostConfig() -> Element {
    let _boosts = vec![Pubkey::from_str("oreFHcE6FvJTrsfaYca4mVeZn7J7T6oZS9FAvW9eg4q").unwrap()];

    // TODO Fetch all boosts
    // TODO Fetch user boosts
    // TODO Display user activated boosts at top
    // TODO In each boost, display user stake, total stake, live multiplier, stake, unstake

    rsx! {
        div {
            class: "flex flex-col gap-8 my-2",
            h2 {
                "Boosts"
            }
            div {
                class: "flex flex-col gap-4",
                Boost {}
            }
        }
    }
}

#[allow(dead_code)]
pub fn Boost() -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-between",
            div {
                class: "flex flex-col gap-2",
                p {
                    "ORE-SOL"
                }
                p {
                    "20 of 100"
                }
            }
            div {
                class: "flex flex-row gap-2",
                p {
                    "1.5x"
                }
            }
        }
    }
}

pub fn PowerLevelConfig() -> Element {
    let mut power_level = use_power_level();
    let max = *WEB_WORKERS as i64;

    rsx! {
        div {
            class: "flex flex-row gap-8 justify-between",
                p {
                    class: "text-gray-300 font-medium text-sm my-auto",
                    "Power"
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
    }
}
