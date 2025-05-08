use std::str::FromStr;

use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use steel::Pubkey;

use crate::{
    components::*,
    config::{Token, UnlistedBoostMeta, UNLISTED_BOOSTS_BY_MINT},
    gateway::GatewayResult,
    hooks::{use_boost_wss, use_stake_wss, use_token_balance_wss},
    pages::{StakeYield, TotalStakers, Weight},
};

#[component]
pub fn Unlisted(mint: String) -> Element {
    let mint = Pubkey::from_str(&mint).unwrap();
    let boost_meta = UNLISTED_BOOSTS_BY_MINT.get(&mint).unwrap();
    let balance = use_token_balance_wss(&mint);
    let boost = use_boost_wss(mint);
    let stake = use_stake_wss(mint);
    let token = use_signal(|| Some(Token::colosseum_nft()));

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Manage your boost position."
            }
            Col {
                class: "gap-16",
                TokenStakeForm {
                    class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                    balance,
                    stake: stake,
                    token,
                }
                AccountMetrics {
                    boost_meta: boost_meta.clone(),
                    boost,
                    stake,
                    token,
                }
                BoostMetrics {
                    boost,
                    boost_meta: boost_meta.clone()
                }
            }
        }
    }
}

#[component]
fn AccountMetrics(
    boost_meta: UnlistedBoostMeta,
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
    token: Signal<Option<Token>>,
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 0,
            Subheading {
                class: "mb-4",
                title: "Account"
            }
            Deposits {
                stake,
                token,
            }
            StakeYield {
                boost,
                stake,
            }
        }
    }
}

#[component]
fn Deposits(stake: Signal<GatewayResult<Stake>>, token: Signal<Option<Token>>) -> Element {
    rsx! {
        TitledRow {
            title: "Deposits",
            description: "The amount of tokens you have deposited in the protocol.",
            value: rsx! {
                if let Some(token) = token.cloned() {
                    if let Ok(stake) = stake.cloned() {
                        if stake.balance > 0 {
                            TokenValueSmall {
                                class: "ml-auto",
                                amount: format!("{:.1$}", stake.balance, 0),
                                ticker: token.ticker,
                                with_decimal_units: false,
                            }
                        } else {
                            NullValue {}
                        }
                    } else {
                        NullValue {}
                    }
                }
            }
        }
    }
}

#[component]
fn BoostMetrics(boost: Signal<GatewayResult<Boost>>, boost_meta: UnlistedBoostMeta) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 0,
            Subheading {
                class: "mb-4",
                title: "Boost"
            }
            TotalStakers {
                boost,
            }
            Weight {
                boost,
            }
        }
    }
}
