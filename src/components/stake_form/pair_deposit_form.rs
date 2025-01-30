use std::str::FromStr;

use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::{spl_associated_token_account::{get_associated_token_address, instruction::{create_associated_token_account, create_associated_token_account_idempotent}}, spl_token::{self, instruction::{close_account, sync_native}}};
use solana_sdk::{native_token::sol_to_lamports, pubkey::Pubkey, system_instruction::transfer, transaction::Transaction};

use crate::{
    components::{submit_transaction, Col, PairWithdrawForm, Row, TransactionStatus}, config::{BoostMeta, LISTED_TOKENS}, gateway::{kamino::KaminoGateway, GatewayError, GatewayResult, UiTokenAmount}, hooks::{use_gateway, use_transaction_status, use_wallet, BoostDeposits, Wallet}
};
use super::common::*;

#[component]
pub fn PairStakeForm(
    class: Option<String>, 
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| StakeTab::Deposit);
    rsx! {
        Col {
            class: "{class}",
            StakeTabs {
                tab: tab
            }
            match *tab.read() {
                StakeTab::Deposit => rsx! {
                    PairDepositForm {
                        boost_meta: boost_meta,
                        boost_deposits: boost_deposits,
                        lp_balance: lp_balance,
                        stake: stake,
                        token_a_balance: token_a_balance,
                        token_b_balance: token_b_balance,
                    }
                },
                StakeTab::Withdraw => rsx! {
                    PairWithdrawForm {
                        boost_meta: boost_meta
                    }
                }
            }
        }
    }
}

#[component]
fn PairDepositForm(
    class: Option<String>, 
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
) -> Element {
    let class = class.unwrap_or_default();
    let wallet = use_wallet();
    let mut stake_amount_a = use_signal::<String>(|| "".to_owned());
    let mut stake_amount_b = use_signal::<String>(|| "".to_owned());
    let transaction_status = use_transaction_status();
 
    // Refresh data, if transaction success
    use_effect(move || {
        if let Some(TransactionStatus::Done(_)) = *transaction_status.read() {
            boost_deposits.restart();
            token_a_balance.restart();
            token_b_balance.restart();
            lp_balance.restart();
            stake.restart();
            stake_amount_a.set("".to_owned());
            stake_amount_b.set("".to_owned());
        }
    });

    // Build the deposit instruction
    let deposit_ix = use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Parse amounts
        let Ok(amount_a) = stake_amount_a.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        let Ok(amount_b) = stake_amount_b.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        if amount_a == 0f64 || amount_b == 0f64 {
            return Err(GatewayError::Unknown);
        }

        // Build the instruction
        use_gateway().build_deposit_instruction(
            boost_meta.lp_id,
            amount_a,
            amount_b,
            authority,
        ).await
    });

    rsx! {
        Col {
            class: "w-full {class}",
            gap: 4,
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
                StakeInputs {
                    mint: boost_meta.pair_mint,
                    amount_a: stake_amount_a,
                    amount_b: stake_amount_b,
                    token_a_balance: token_a_balance,
                    token_b_balance: token_b_balance,
                    boost_deposits: boost_deposits,
                }
            }
            // StakeDetails {}
            SubmitButton {
                enabled: if let Some(Ok(_ix)) = deposit_ix.cloned() {
                    true
                } else {
                    false
                },
                onclick: move |_| {
                    // Compile instructions
                    let mut ixs = vec![];

                    // Return if wallet is not connected
                    let Wallet::Connected(authority) = *wallet.read() else {
                        return;
                    };

                    // Return if amount is not valid
                    let Ok(amount_a_f64) = stake_amount_a.cloned().parse::<f64>() else {
                        return;
                    };

                    // Create ata for lp shares, if needed
                    if let Some(Ok(_)) = lp_balance.cloned() {
                        // Do nothing
                    } else {
                        ixs.push(
                            create_associated_token_account(&authority, &authority, &boost_meta.lp_mint, &spl_token::ID)
                        );
                    }

                    // Handle wrapped SOL, if needed
                    let token_a_ata = get_associated_token_address(&authority, &boost_meta.pair_mint);
                    let is_sol = boost_meta.pair_mint == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
                    if is_sol {
                        ixs.push(
                            create_associated_token_account_idempotent(&authority, &authority, &boost_meta.pair_mint, &spl_token::ID)
                        );
                        ixs.push(
                            transfer(&authority, &token_a_ata, sol_to_lamports(amount_a_f64))
                        );
                        ixs.push(
                            sync_native(&spl_token::ID, &token_a_ata).unwrap()
                        );
                    }

                    // Append deposit instruction
                    let Some(Ok(deposit_ix)) = deposit_ix.cloned() else {
                        return;
                    };
                    ixs.push(deposit_ix);

                    // Close the wSOL ata
                    if is_sol {
                        ixs.push(
                            close_account(&spl_token::ID, &token_a_ata, &authority, &authority, &[&authority]).unwrap()
                        );
                    }

                    // Open the stake account, if needed
                    if let Some(Ok(_stake)) = stake.read().as_ref() {
                        // Do nothing
                    } else {
                        ixs.push(ore_boost_api::sdk::open(authority, authority, boost_meta.lp_mint));
                    }

                    // Stake LP tokens into boost program
                    ixs.push(
                        ore_boost_api::sdk::deposit(authority, boost_meta.lp_mint, u64::MAX)
                    );

                    // Submit transaction
                    let tx = Transaction::new_with_payer(&ixs, Some(&authority));
                    submit_transaction(tx.into());
                }
            }
        }
    }
}

#[component]
fn StakeInputs(
    mint: Pubkey,
    amount_a: Signal<String>,
    amount_b: Signal<String>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
) -> Element {
    let token = LISTED_TOKENS.get(&mint).unwrap();
    rsx! {
        Col {
            class: "w-full p-4",
            gap: 4,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-midEmphasis my-auto pl-1",
                    "Deposit"
                }
                MaxButton {
                    amount_a: amount_a,
                    amount_b: amount_b,
                    token_a_balance: token_a_balance,
                    token_b_balance: token_b_balance,
                    boost_deposits: boost_deposits,
                }
            }
            Col {
                gap: 4,
                Row {
                    gap: 4,
                    Row {
                        class: "my-auto",
                        gap: 2,
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: "{token.image}",
                        }
                        span {
                            class: "font-semibold my-auto",
                            "{token.ticker}"
                        }
                    }
                    input {
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: amount_a.cloned(),
                        oninput: move |e| {
                            let Some(Ok(deposits)) = boost_deposits.cloned() else {
                                return;
                            };

                            let ratio = deposits.balance_a / deposits.balance_b;

                            let val = e.value();
                            if val.len().eq(&0) {
                                amount_a.set(val.clone());
                                amount_b.set(val);
                                return;
                            }

                            if let Ok(val_f64) = val.parse::<f64>() {
                                if val_f64 >= 0f64 {
                                    amount_a.set(val);
                                    amount_b.set((val_f64 / ratio).to_string());
                                } else {
                                    amount_a.set("".to_string());
                                    amount_b.set("".to_string());
                                }
                            } else {
                                amount_a.set(val[..val.len()-1].to_string());
                            }
                        }
                    }
                }
                Row {
                    gap: 4,
                    Row {
                        class: "my-auto",
                        gap: 2,
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: asset!("/public/icon.png"),
                        }
                        span {
                            class: "font-semibold my-auto",
                            "ORE"
                        }
                    }
                    input {
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: amount_b.cloned(),
                        oninput: move |e| {
                            let Some(Ok(deposits)) = boost_deposits.cloned() else {
                                return;
                            };

                            let ratio = deposits.balance_a / deposits.balance_b;

                            let val = e.value();
                            if val.len().eq(&0) {
                                amount_a.set(val.clone());
                                amount_b.set(val);
                                return;
                            }

                            if let Ok(val_f64) = val.parse::<f64>() {
                                if val_f64 >= 0f64 {
                                    amount_a.set((val_f64 * ratio).to_string());
                                    amount_b.set(val);
                                } else {
                                    amount_a.set("".to_string());
                                    amount_b.set("".to_string());
                                }
                            } else {
                                amount_b.set(val[..val.len()-1].to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}


#[component]
fn MaxButton(
    amount_a: Signal<String>,
    amount_b: Signal<String>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
) -> Element {
    rsx! {
        button {
            class: "text-xs my-auto py-1 px-1 font-medium text-elements-lowEmphasis hover:text-elements-highEmphasis hover:cursor-pointer",
            onclick: move |_| {
                let Some(Ok(token_a_balance)) = token_a_balance.cloned() else {
                    return;
                };
                let Some(Ok(token_b_balance)) = token_b_balance.cloned() else {
                    return;
                };
                let Some(Ok(boost_deposits)) = boost_deposits.cloned() else {
                    return;
                };

                let token_a_amount = token_a_balance.ui_amount.unwrap_or(0.0);
                let token_b_amount = token_b_balance.ui_amount.unwrap_or(0.0);

                let ratio = boost_deposits.balance_a / boost_deposits.balance_b;

                let max_b = token_a_amount / ratio;
                let max_a = token_b_amount * ratio;

                if max_a <= token_a_amount {
                    amount_a.set(max_a.to_string());
                    amount_b.set(token_b_amount.to_string());
                } else {
                    amount_a.set(token_a_amount.to_string());
                    amount_b.set(max_b.to_string());
                }
            },
            "Max"
        }
    }
}