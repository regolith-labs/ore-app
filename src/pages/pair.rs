use std::str::FromStr;

use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::{spl_associated_token_account, spl_token::amount_to_ui_amount_string};
use solana_sdk::transaction::Transaction;
use steel::Pubkey;

use crate::{components::{submit_transaction, Col, Heading, NullValue, OreValueSmall, PairStakeForm, Row, TokenValueSmall, TransactionStatus, UsdValueSmall}, config::{BoostMeta, LpType, LISTED_BOOSTS_BY_MINT}, gateway::{GatewayResult, UiTokenAmount}, hooks::{use_boost, use_boost_deposits, use_kamino_global_config, use_ore_balance, use_stake, use_token_balance, use_transaction_status, use_wallet, BoostDeposits, Wallet}, pages::ClaimButton};

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
                subtitle: "Manage your stake position."
            }
            Col {
                gap: 16,
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
                    ore_balance,
                    stake
                }
                BoostMetrics {
                    boost,
                    boost_deposits,
                    boost_meta: boost_meta.clone()
                }
                if let Some(Ok(lp_token_balance)) = lp_balance.read().as_ref() {  
                    if lp_token_balance.ui_amount.unwrap_or(0.0) > 0.0 {
                        UnstakedMetrics {
                            boost_meta: boost_meta.clone(),
                            lp_balance: lp_balance,
                            boost_deposits: boost_deposits,
                            boost: boost,
                            stake: stake,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AccountMetrics(
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
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 4,
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
                if let Some(Ok(stake)) = stake.read().as_ref() {
                    if stake.balance > 0 {
                        span {
                            class: "text-elements-highEmphasis font-medium",
                            "{stake.balance}"
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
                        span {
                            class: "text-elements-highEmphasis font-medium",
                            "{stake.balance_pending}"
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
                    let mut ixs = vec![];
                    let Wallet::Connected(authority) = *wallet.read() else {
                        return;
                    };
                    let Some(Ok(stake)) = *stake.read() else {
                        return;
                    };
                    let beneficiary = spl_associated_token_account::get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);
                    ixs.push(ore_boost_api::sdk::claim(authority, beneficiary, ore_api::consts::MINT_ADDRESS, stake.rewards));
                    let transaction = Transaction::new_with_payer(&ixs, Some(&authority));
                    submit_transaction(transaction.into());
                },
            }
        }
    }
}

#[component]
fn BoostMetrics(
    boost: Resource<GatewayResult<Boost>>,
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 4,
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl",
                "Totals"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Deposits"
                }
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    OreValueSmall {
                        ui_amount_string: boost_deposits.balance_b.to_string(),
                    }
                } else {
                    NullValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Liquidity"
                }
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    TokenValueSmall {
                        amount: boost_deposits.balance_a.to_string(),
                        ticker: boost_deposits.token_a.clone(),
                    }
                } else {
                    NullValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Operator"
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
                    "Stakers"
                }
                if let Some(Ok(boost)) = boost.read().as_ref() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.total_stakers}"
                    }
                } else {
                    NullValue {}
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
                    NullValue {}
                }   
            }
        }
    }
}

#[component]
fn UnstakedMetrics(
    boost_meta: BoostMeta,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let wallet = use_wallet();
    let transaction_status = use_transaction_status();

    // Refresh data if successful transaction
    use_effect(move || {
        if let Some(TransactionStatus::Done(_)) = *transaction_status.read() {
            lp_balance.restart();
            boost_deposits.restart();
            boost.restart();
            stake.restart();
        }
    });

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 4,
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl",
                "Unstaked"
            }
            span {
                class: "text-elements-lowEmphasis px-2",
                "You have unstaked LP tokens in your connected wallet. To deposit these tokens and earn boosted yield, use the button below."
            }
            button {
                class: "h-12 w-full rounded-full controls-secondary",
                onclick: move |_| {
                    // Compile instructions
                    let mut ixs = vec![];

                    // Get the wallet authority
                    let Wallet::Connected(authority) = *wallet.read() else {
                        return;
                    };

                    // Check if LP tokens are in the wallet
                    let Some(Ok(_lp_token_balance)) = lp_balance.cloned() else {
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
