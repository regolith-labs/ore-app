use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::{spl_associated_token_account, spl_token::amount_to_ui_amount_string};
use solana_sdk::transaction::Transaction;

use crate::{components::*, gateway::{ui_token_amount::UiTokenAmount, GatewayResult}, hooks::{use_boost, use_ore_balance, use_stake, use_transaction_status, use_wallet, Wallet}};

pub fn Vault() -> Element {
    let ore_balance = use_ore_balance();
    let ore_boost = use_boost(ore_api::consts::MINT_ADDRESS);
    let ore_stake = use_stake(ore_api::consts::MINT_ADDRESS);

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
                    ore_balance,
                    ore_stake,
                }
                VaultPosition {
                    ore_balance,
                    ore_stake,
                }
                VaultTotals {
                    ore_boost,
                }
            }
        }
    }
}

#[component]
fn VaultPosition(
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>
) -> Element {
    let wallet = use_wallet();
    let mut enabled = use_signal(|| false);
    let transaction_status = use_transaction_status();

    // Enable claim button
    use_effect(move || {
        if let Some(Ok(stake)) = ore_stake.read().as_ref() {
            enabled.set(stake.rewards > 0);
        } else {
            enabled.set(false);
        };
    });

    // Refresh data if successful transaction
    use_effect(move || {
        if let Some(TransactionStatus::Done(_)) = *transaction_status.read() {
            ore_balance.restart();
            ore_stake.restart();
        }
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
                if let Some(Ok(stake)) = ore_stake.read().as_ref() {
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
                if let Some(Ok(stake)) = ore_stake.read().as_ref() {
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
                if let Some(Ok(stake)) = ore_stake.read().as_ref() {
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
                    let Some(Ok(stake)) = *ore_stake.read() else {
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

#[component]
fn VaultTotals(
    ore_boost: Resource<GatewayResult<Boost>>
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 4,
            span {
                class: "text-elements-highEmphasis font-medium text-2xl",
                "Totals"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Stakers"
                }
                if let Some(Ok(boost)) = ore_boost.read().as_ref() {
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
                if let Some(Ok(boost)) = ore_boost.read().as_ref() {
                    OreValueSmall {
                        class: "text-elements-highEmphasis",
                        ui_amount_string: amount_to_ui_amount_string(boost.total_deposits, TOKEN_DECIMALS),
                    }
                } else {
                    NullValue {}
                }   
            }
        }
    }
}