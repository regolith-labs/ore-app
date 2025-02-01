use std::str::FromStr;

use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::{spl_associated_token_account, spl_token::amount_to_ui_amount_string};
use solana_sdk::transaction::Transaction;
use steel::Pubkey;

use crate::{components::{submit_transaction, Col, Heading, LoadingValue, NullValue, OreValueSmall, PairStakeForm, PairStakeValue, PairValue, Row, TransactionStatus, UsdValueSmall}, config::{BoostMeta, LpType, LISTED_BOOSTS_BY_MINT}, gateway::{GatewayResult, UiTokenAmount}, hooks::{use_boost, use_boost_deposits, use_ore_balance, use_stake, use_token_balance, use_transaction_status, use_wallet, BoostDeposits, Wallet}, pages::ClaimButton};

#[component]
pub fn Pair(lp_mint: String) -> Element {
    let lp_mint = Pubkey::from_str(&lp_mint).unwrap();
    let boost_meta = LISTED_BOOSTS_BY_MINT.get(&lp_mint).unwrap();
    let boost = use_boost(lp_mint);
    let boost_deposits = use_boost_deposits(boost_meta.clone());
    let stake = use_stake(lp_mint);
    let pair_balance = use_token_balance(boost_meta.pair_mint);
    let ore_balance = use_ore_balance();
    let lp_balance = use_token_balance(lp_mint);

    // TODO Get the boost
    // TODO Show error if boost is not listed
    
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: boost_meta.name.clone(),
                subtitle: "Manage your liquidity pair."
            }
            Col {
                class: "w-full h-full gap-16",
                PairStakeForm {
                    class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                    boost_meta: boost_meta.clone(),
                    boost_deposits: boost_deposits,
                    lp_balance: lp_balance,
                    stake: stake,
                    token_a_balance: pair_balance,
                    token_b_balance: ore_balance,
                }
                AccountMetrics {
                    boost_meta: boost_meta.clone(),
                    boost_deposits: boost_deposits,
                    ore_balance,
                    lp_balance,
                    stake
                }
                SummaryMetrics {
                    boost,
                    boost_deposits,
                    boost_meta: boost_meta.clone()
                }
            }
        }
    }
}

#[component]
fn AccountMetrics(
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let wallet = use_wallet();
    let mut enabled = use_signal(|| false);
    let transaction_status = use_transaction_status();

    // Enable claim button
    use_effect(move || {
        if let Some(Ok(stake)) = stake.read().as_ref() {
            enabled.set(stake.rewards > 0);
        } else {
            enabled.set(false);
        };
    });

    // Refresh data if successful transaction
    use_effect(move || {
        if let Some(TransactionStatus::Done(_)) = *transaction_status.read() {
            ore_balance.restart();
            stake.restart();
        }
    });

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8 gap-8",
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl",
                "Account"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Deposits"
                }
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    if let Some(stake) = stake.read().as_ref() {
                        if let Ok(stake) = stake {
                            if stake.balance > 0 {
                                PairStakeValue {
                                    shares: stake.balance,
                                    boost_deposits: boost_deposits.clone(),
                                    small_units: Some(true),
                                }
                            } else {
                                NullValue {}
                            }
                        } else {
                            NullValue {}
                        }
                    } else {
                        LoadingValue {}
                    }
                } else {
                    LoadingValue {}
                }
            }
            if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                if let Some(Ok(stake)) = stake.read().as_ref() {
                    if stake.balance_pending > 0 {
                        Row {
                            class: "w-full justify-between px-4",
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Deposits (pending)"
                            }
                            PairStakeValue {
                                shares: stake.balance_pending,
                                boost_deposits: boost_deposits.clone(),
                                small_units: Some(true),
                            }
                        }
                    }
                }
            }
            if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                if let Some(Ok(lp_balance)) = lp_balance.read().as_ref() {
                    if lp_balance.ui_amount.unwrap_or(0.0) > 0.0 {
                        Row {
                            class: "w-full justify-between px-4",
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Unstaked"
                            }
                            PairStakeValue {
                                shares: lp_balance.amount.parse::<u64>().unwrap_or(0),
                                boost_deposits: boost_deposits.clone(),
                                small_units: Some(true),
                            }
                            // span {
                            //     class: "text-elements-highEmphasis font-medium",
                            //     "{lp_balance.ui_amount_string} {boost_meta.ticker}"
                            // }
                        }
                        button {
                            class: "h-12 w-full rounded-full controls-tertiary",
                            onclick: move |_| {
                                // Compile instructions
                                let mut ixs = vec![];
            
                                // Get the wallet authority
                                let Wallet::Connected(authority) = *wallet.read() else {
                                    return;
                                };
            
                                // Open the stake account, if needed
                                if let Some(Ok(_stake)) = stake.read().as_ref() {
                                    // Do nothing
                                } else {
                                    ixs.push(ore_boost_api::sdk::open(authority, authority, boost_meta.lp_mint));
                                }
            
                                // Deposit LP tokens
                                ixs.push(ore_boost_api::sdk::deposit(authority, boost_meta.lp_mint, u64::MAX));
                                let transaction = Transaction::new_with_payer(&ixs, Some(&authority));
                                submit_transaction(transaction.into());
                            },
                            span {
                                class: "mx-auto my-auto font-semibold",
                                "Deposit {boost_meta.ticker}"
                            }
                        }
                    }
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Yield"
                }
                if let Some(stake) = stake.read().as_ref() {
                    if let Ok(stake) = stake {
                        if stake.rewards > 0 {
                            OreValueSmall {
                                class: "text-elements-gold",
                                ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                                small_units: true,
                            }
                        } else {
                            NullValue {}
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    LoadingValue {}
                }
            }
            ClaimButton {
                enabled: enabled.clone(),
                onclick: move |_| {
                    let mut ixs = vec![];
                    let Wallet::Connected(authority) = *wallet.read() else {
                        return;
                    };
                    let Some(Ok(stake)) = *stake.read() else {
                        return;
                    };
                    let beneficiary = spl_associated_token_account::get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);
                    ixs.push(ore_boost_api::sdk::claim(authority, beneficiary, boost_meta.lp_mint, stake.rewards));
                    let transaction = Transaction::new_with_payer(&ixs, Some(&authority));
                    submit_transaction(transaction.into());
                },
            }
        }
    }
}

#[component]
fn SummaryMetrics(
    boost: Resource<GatewayResult<Boost>>,
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 8,
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl",
                "Boost"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Multiplier"
                }
                if let Some(Ok(boost)) = boost.read().as_ref() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.multiplier as f64 / ore_boost_api::consts::BOOST_DENOMINATOR as f64}x"
                    }
                } else {
                    LoadingValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Protocol"
                }
                a {
                    class: "text-elements-highEmphasis font-medium hover:underline",
                    href: match boost_meta.lp_type {
                        LpType::Kamino => format!("https://app.kamino.finance/liquidity/{}", boost_meta.lp_id),
                        LpType::Meteora => format!("https://app.meteora.ag/pools/{}", boost_meta.lp_id),
                    },
                    target: "_blank",
                    "{boost_meta.lp_type}"
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Total deposits"
                }
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    PairValue {
                        boost_deposits: boost_deposits.clone(),
                        small_units: true,
                    }
                } else {
                    LoadingValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Total stakers"
                }
                if let Some(Ok(boost)) = boost.read().as_ref() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.total_stakers}"
                    }
                } else {
                    LoadingValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "TVL"
                }
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    UsdValueSmall {
                        amount: boost_deposits.total_value_usd.to_string(),
                    }
                } else {
                    LoadingValue {}
                }   
            }
        }
    }
}
