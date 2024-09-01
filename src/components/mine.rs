use std::str::FromStr;

use dioxus::prelude::*;
use ore_boost_api::state::Boost;
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
                div {
                    class: "flex flex-col gap-4 my-2",
                    // MultiplierDisplay {}
                    PowerLevelConfig {}
                    UnclaimedBalanceDisplay {}
                }
            }
            BoostConfig {}
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

pub fn UnclaimedBalanceDisplay() -> Element {
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

pub fn BoostConfig() -> Element {
    let _boosts = vec![Pubkey::from_str("oreFHcE6FvJTrsfaYca4mVeZn7J7T6oZS9FAvW9eg4q").unwrap()];

    // TODO Fetch all boosts
    // TODO Fetch user boosts
    // TODO Display user activated boosts at top
    // TODO In each boost, display user stake, total stake, live multiplier, stake, unstake

    let boosts = [
        (
            "ORE-SOL",
            Boost {
                bump: 0,
                mint: Pubkey::new_unique(),
                multiplier: 4,
                total_stake: 1000,
            },
            10,
        ),
        (
            "ORE-USDT",
            Boost {
                bump: 0,
                mint: Pubkey::new_unique(),
                multiplier: 4,
                total_stake: 2000,
            },
            0,
        ),
        (
            "ORE-ISC",
            Boost {
                bump: 0,
                mint: Pubkey::new_unique(),
                multiplier: 4,
                total_stake: 3000,
            },
            0,
        ),
        (
            "ORE",
            Boost {
                bump: 0,
                mint: Pubkey::new_unique(),
                multiplier: 2,
                total_stake: 4000,
            },
            100,
        ),
    ];

    // rsx! {
    //     div {
    //         class: "table w-full",
    //         div {
    //             class: "table-header-group w-full",
    //             div {
    //                 class: "table-row",
    //                 div { class: "table-cell text-left ...", "Song" }
    //                 div { class: "table-cell text-left ...", "Artist" }
    //                 div { class: "table-cell text-left ...", "Year" }
    //             }
    //         }
    //         div {
    //             class: "table-row-group w-full",
    //             div {
    //                 class: "table-row",
    //                 div { class: "table-cell ...", "The Sliding" }
    //                 div { class: "table-cell ...", "Malcolm Lockyer" }
    //                 div { class: "table-cell ...", "1961" }
    //             }
    //             div {
    //                 class: "table-row",
    //                 div { class: "table-cell ...", "Witchy Woman" }
    //                 div { class: "table-cell ...", "The Eagles" }
    //                 div { class: "table-cell ...", "1972" }
    //             }
    //             div {
    //                 class: "table-row",
    //                 div { class: "table-cell ...", "Shining Star" }
    //                 div { class: "table-cell ...", "Earth, Wind, and Fire" }
    //                 div { class: "table-cell ...", "1975" }
    //             }
    //         }
    //     }
    // }

    rsx! {
        div {
            class: "flex flex-col gap-4 my-2 border-collapse",
            p {
                class: "text-3xl font-semibold",
                "Boosts"
            }
            p {
                "Boosts are multipliers you can earn by staking community aligned tokens."
            }
            table {
                class: "border-collapse",
                tr {
                    class: "h-10 text-gray-300 text-sm",
                    th { class: "text-left font-medium", "Asset" }
                    th { class: "text-right font-medium", "Multiplier" }
                    th { class: "text-right font-medium", "Stake" }
                    th { class: "text-right font-medium", "Active" }
                }
                for boost in boosts {
                    BoostRowCore {
                        name: boost.0,
                        boost: boost.1,
                        stake: boost.2
                    }
                    // BoostRowExtended {
                    //     name: boost.0,
                    //     boost: boost.1,
                    //     stake: boost.2
                    // }
                }
            }
        }
    }
}

#[component]
pub fn BoostRowCore(name: String, boost: Boost, stake: u64) -> Element {
    let mut hover = use_signal(|| false);
    let arrow_class = if *hover.read() { "" } else { "opacity-0" };
    rsx! {
        tr {
            class: format!(
                "{}",
                if stake.gt(&0) { "h-20 pt-2" } else { "h-16" },
            ),
            td {
                class: "align-top hover:cursor-pointer",
                onmouseenter: move |_| { hover.set(true) },
                onmouseleave: move |_| { hover.set(false) },
                div {
                    class: "flex flex-row gap-3",
                    div {
                        class: "w-8 h-8 rounded-full mb-auto mt-2 bg-gray-800"
                    }
                    div {
                        class: "flex flex-col mb-auto mt-3 gap-1",
                        span {
                            class: "font-semibold",
                            "{name} "
                            span {
                                class: "transition-opacity {arrow_class}",
                                "→"
                            }
                        }
                        if stake.gt(&0) {
                            span {
                                class: "text-sm text-gray-300",
                                "Balance: 0"
                            }
                        }
                    }
                }
            }
            td {
                class: "text-right font-base align-top",
                div {
                    class: "flex flex-col gap-1",
                    span {
                        class: "mt-3 mb-auto",
                        "{boost.multiplier}x"
                    }
                    if stake.gt(&0) {
                        span {
                            class: "text-sm text-gray-300",
                            "1.0004x"
                        }
                    }
                }
            }
            td {
                class: "text-right font-base align-top",
                div {
                    class: "flex flex-col gap-1",
                    span {
                        class: "mt-3 mb-auto",
                        "{boost.total_stake}"
                    }
                    if stake.gt(&0) {
                        span {
                            class: "text-sm text-gray-300",
                            "1000"
                        }
                    }
                }
            }
            td {
                class: "text-right align-top",
                div {
                    class: "flex",
                    ToggleSwitch {
                        class: "mt-3 mb-auto ml-auto",
                        enabled: stake.gt(&0),
                        is_checked: false,
                        ontoggle: move |_| {
                             // TODO
                        }
                    }
                }
            }
        }
    }
}

// Name: 1
// Image: 1
// Multiplier: 1
// Total stake: 2
// Enabled: Action 1

// (optional)
// My wallet balance: 3
// My stake
// My multiplier

// (click into)
// Total unique stakers
// Description: 2
// Stake: Action 3
// Unstake: Action 3

#[component]
fn ToggleSwitch(
    class: Option<String>,
    enabled: bool,
    is_checked: bool,
    ontoggle: EventHandler<MouseEvent>,
) -> Element {
    let mut on = use_signal(|| is_checked);
    rsx! {
        div {
            class: format!(
                "relative inline-flex items-center {} {}",
                if enabled { "cursor-pointer" } else { "opacity-50" },
                class.unwrap_or("".to_string())
            ),
            div {
                class: format!(
                    "w-11 h-6 rounded-full transition {}",
                    if *on.read() { "bg-green-500" } else { "bg-gray-900" }
                ),
                onclick: move |e| {
                    if enabled {
                        let current = *on.read();
                        on.set(!current);
                        ontoggle.call(e)
                    }
                },
                div {
                    class: format!(
                        "w-4 h-4 mt-1 bg-white rounded-full shadow-md transform transition {}",
                        if *on.read() { "translate-x-6" } else { "translate-x-1" }
                    ),
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
