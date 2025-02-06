use std::collections::HashMap;

use dioxus::prelude::*;
use steel::Pubkey;
use ore_boost_api::state::Stake;

use crate::{
    components::*, 
    gateway::GatewayResult, 
    hooks::{on_transaction_done, use_boost_claim_all_transaction, use_liquidity_pairs, use_net_deposits, use_net_yield, use_stake_accounts, LiquidityPair}
};

pub fn Stake() -> Element {
    let stake_accounts = use_stake_accounts();
    let liquidity_pairs = use_liquidity_pairs();

    // Refresh stake accounts after transaction
    let mut stake_accounts_mut = stake_accounts.clone();
    on_transaction_done(move |_sig| {
        for (_mint, stake_account) in stake_accounts_mut.iter_mut() {
            stake_account.restart();
        }
    });

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Provide market liquidity and earn yield."
            }
            Col {
                gap: 16,
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
}

#[component]
fn AccountSummary(
    stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>,
    liquidity_pairs: HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>>
) -> Element {
    let net_deposits = use_net_deposits(stake_accounts.clone(), liquidity_pairs.clone());
    let net_yield = use_net_yield(stake_accounts.clone());
    rsx! {
        Col {
            class: "mx-auto w-full px-5 sm:px-8 justify-between",
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl mb-6",
                "Account"
            }
            Col {
                class: "md:flex-row w-full justify-between px-0 sm:px-2",
                gap: 8,
                Col {
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "Net deposits"
                    }
                    if let Some(Ok(net_deposits)) = net_deposits.cloned() {
                        OreValue {
                            ui_amount_string: net_deposits.ui_amount_string,
                            with_decimal_units: true,
                            size: TokenValueSize::Large,
                        }
                    } else {
                        LoadingValue {}
                    }
                }
                Col {
                    class: "min-w-56",
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis font-medium md:text-right",
                        "Net yield"
                    }
                    if let Some(Ok(net_yield)) = net_yield.cloned() {
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
            Row {
                class: "mx-auto w-full mt-8 md:mt-4 md:justify-end",
                ClaimButton {
                    stake_accounts: stake_accounts.clone()
                }
            }
        }
    }
}

#[component]
fn ClaimButton(stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>) -> Element {
    let tx = use_boost_claim_all_transaction(stake_accounts);
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
                    submit_transaction(tx);
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
