use std::collections::HashMap;

use dioxus::prelude::*;
use steel::Pubkey;
use ore_boost_api::state::Stake;

use crate::{components::*, gateway::GatewayResult, hooks::{use_liquidity_pairs, use_net_deposits, use_net_yield, use_stake_accounts, LiquidityPair}, route::Route};

pub fn Stake() -> Element {
    let stake_accounts = use_stake_accounts();
    let liquidity_pairs = use_liquidity_pairs();
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 16,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Provide liquidity for traders and earn yield."
            }
            AccountSummary {
                stake_accounts: stake_accounts.clone(),
                liquidity_pairs: liquidity_pairs.clone()
            }
            StakeTable {
                stake_accounts: stake_accounts,
                liquidity_pairs: liquidity_pairs
            }
        }
    }
}

#[component]
fn AccountSummary(
    stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>,
    liquidity_pairs: HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>>
) -> Element {
    let net_deposits = use_net_deposits(stake_accounts.clone(), liquidity_pairs);
    let net_yield = use_net_yield(stake_accounts);
    rsx! {
        Col {
            class: "mx-auto w-full px-5 sm:px-8 justify-between",
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl mb-6",
                "Account"
            }
            Row {
                class: "hidden sm:flex w-full justify-between px-0 sm:px-2",
                Col {
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "Net deposits"
                    }
                    if let Some(Ok(net_deposits)) = net_deposits.cloned() {
                        OreValue {
                            ui_amount_string: net_deposits.ui_amount_string,
                        }
                    } else {
                        LoadingValue {}
                    }
                }
                Col {
                    class: "min-w-56",
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis font-medium text-right",
                        "Net yield"
                    }
                    if let Some(Ok(net_yield)) = net_yield.cloned() {
                        OreValue {
                            class: "text-elements-gold ml-auto",
                            ui_amount_string: net_yield.ui_amount_string,
                        }
                    } else {
                        LoadingValue {}
                    }
                }
            }
            ActionButtons {}
        }
    }
}

fn ActionButtons() -> Element {
    rsx! {
        Row {
            class: "mx-auto w-full mt-4 justify-end",
            ClaimButton {}
        }
    }
}

fn ClaimButton() -> Element {
    rsx! {
        Link {
            to: Route::Landing {},
            class: "flex flex-row h-10 w-min controls-gold rounded-full px-4 gap-2",
            // CircleStackIcon {
            //     class: "h-5 w-5 mx-auto my-auto"
            // }
            span {
                class: "my-auto text-nowrap",
                "Claim"
            }
        }
    }
}


fn _YieldOverview() -> Element {
    // TODO Get all stake accounts
    // TODO Calculate total claimable yield
    // TODO Provide claim button
    rsx! {
        Row {
            class: "mx-5 sm:mx-8 py-8 justify-between",
            // div {
            //     class: "flex w-full",
            //     OreValue {
            //         class: "mx-auto my-auto",
            //         ui_amount_string: "2.324330".to_string(),
            //     }
            // }
            div {
                class: "flex w-full",
                span {
                    class: "text-elements-midEmphasis font-bold text-2xl sm:text-3xl my-auto mx-auto",
                    "0.04%"
                }   
            }
            div {
                class: "flex w-full",
                OreValueGold {
                    class: "mx-auto my-auto",
                    ui_amount_string: "2.324330".to_string(),
                }   
            }
        }
    }
}
