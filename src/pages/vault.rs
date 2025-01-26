use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

use crate::{components::{Col, Heading, OreValueSmall, Row, VaultStakeForm}, hooks::{use_boost, use_stake}};

pub fn Vault() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Vault",
                subtitle: "Stake unpaired ORE to earn the idle yield rate."
            }
            Col {
                gap: 16,
                VaultStakeForm {
                    class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                }
                PositionSummary {}
                VaultSummary {}
            }
        }
    }
}

fn PositionSummary() -> Element {
    let stake = use_stake(ore_api::consts::MINT_ADDRESS);
    let mut enabled = use_signal(|| false);

    // Enable claim button
    use_effect(move || {
        if let Some(Ok(stake)) = stake.read().as_ref() {
            enabled.set(stake.rewards > 0);
        } else {
            enabled.set(false);
        };
    });

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 4,
            span {
                class: "text-elements-highEmphasis font-medium text-2xl",
                "Account"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Deposits"
                }
                if let Some(Ok(stake)) = stake.read().as_ref() {
                    if stake.balance > 0 {
                        OreValueSmall {
                            class: "text-elements-highEmphasis",
                            ui_amount_string: amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS),
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    NullValue {}
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Pending deposits"
                }
                if let Some(Ok(stake)) = stake.read().as_ref() {
                    if stake.balance_pending > 0 {
                        OreValueSmall {
                            class: "text-elements-highEmphasis",
                            ui_amount_string: amount_to_ui_amount_string(stake.balance_pending, TOKEN_DECIMALS),
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    NullValue {}
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Yield"
                }
                if let Some(Ok(stake)) = stake.read().as_ref() {
                    if stake.rewards > 0 {
                        OreValueSmall {
                            class: "text-elements-gold",
                            ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    NullValue {}
                }
            }
            ClaimButton {
                enabled: enabled.clone(),
                onclick: move |_| {
                    // TODO: Implement claim logic
                },
            }
        }
    }
}

fn NullValue() -> Element {
    rsx! {
        span {
            class: "text-elements-midEmphasis font-medium",
            "–"
        }
    }
}

#[component]
fn ClaimButton(enabled: Signal<bool>, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-gold",
            disabled: !*enabled.read(),
            onclick: onclick,
            span {
                class: "mx-auto my-auto",
                "Claim"
            }
        }
    }
}

fn VaultSummary() -> Element {
    let boost = use_boost(ore_api::consts::MINT_ADDRESS);

    let boost_deposits = if let Some(Ok(boost)) = boost.read().as_ref() {
        amount_to_ui_amount_string(boost.total_deposits, TOKEN_DECIMALS)
    } else {
        "0.000".to_string()
    };

    let total_stakers = if let Some(Ok(boost)) = boost.read().as_ref() {
        boost.total_stakers
    } else {
        0
    };

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 4,
            span {
                class: "text-elements-highEmphasis font-medium text-2xl",
                "Overall"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Total deposits"
                }
                OreValueSmall {
                    class: "text-elements-highEmphasis",
                    ui_amount_string: boost_deposits,
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Total stakers"
                }
                span {
                    class: "text-elements-highEmphasis font-medium",
                    "{total_stakers}"
                }
            }
        }
    }
}