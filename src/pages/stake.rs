use dioxus::prelude::*;
use ore_types::request::TransactionType;

use crate::{
    components::*,
    hooks::{use_boost_claim_all_transaction, use_net_deposits, use_net_yield},
};

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Provide liquidity and earn yield."
            }
            Col {
                gap: 16,
                AccountSummary {}
                StakeTable {}
            }
        }
    }
}

fn AccountSummary() -> Element {
    rsx! {
        Col {
            class: "mx-auto w-full px-5 sm:px-8 justify-between",
            gap: 8,
            Subheading {
                title: "Account"
            }
            Col {
                class: "md:flex-row w-full justify-between px-0 sm:px-2",
                gap: 8,
                NetDeposits {}
                NetYield {}
            }
            Row {
                class: "mx-auto w-full md:justify-end",
                ClaimButton {}
            }
        }
    }
}

fn NetDeposits() -> Element {
    let net_deposits = use_net_deposits();
    rsx! {
        Col {
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium",
                "Net deposits"
            }
            if let Ok(net_deposits) = net_deposits.cloned() {
                OreValue {
                    ui_amount_string: net_deposits.ui_amount_string,
                    with_decimal_units: true,
                    size: TokenValueSize::Large,
                }
            } else {
                LoadingValue {}
            }
        }
    }
}

fn NetYield() -> Element {
    let net_yield = use_net_yield();
    rsx! {
        Col {
            class: "min-w-56",
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium md:text-right",
                "Net yield"
            }
            if let Ok(net_yield) = net_yield.cloned() {
                OreValue {
                    class: "md:text-right md:ml-auto",
                    ui_amount_string: net_yield.ui_amount_string,
                    with_decimal_units: true,
                    size: TokenValueSize::Large,
                    gold: true,
                }
            } else {
                LoadingValue {}
            }
        }
    }
}

fn ClaimButton() -> Element {
    // Build claim all transaction
    let tx = use_boost_claim_all_transaction();
    let is_enabled = if let Some(Ok(_)) = *tx.read() {
        true
    } else {
        false
    };

    rsx! {
        button {
            disabled: !is_enabled,
            onclick: move |_| {
                if let Some(Ok(tx)) = tx.cloned() {
                    submit_transaction(tx, TransactionType::BoostClaim);
                }
            },
            class: "flex flex-row h-12 w-full md:w-min controls-gold rounded-full px-8",
            span {
                class: "my-auto mx-auto text-nowrap",
                "Claim"
            }
        }
    }
}
