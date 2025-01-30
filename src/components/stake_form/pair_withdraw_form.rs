use std::str::FromStr;

use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::{spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account_idempotent}, spl_token::{self, instruction::{close_account, sync_native}}};
use solana_sdk::{hash::Hash, message::{v0, VersionedMessage}, transaction::{Transaction, VersionedTransaction}};
use steel::Pubkey;

use crate::{components::{stake_form::common::WithdrawButton, submit_transaction, Col, Row}, config::{BoostMeta, LISTED_TOKENS, LISTED_TOKENS_BY_TICKER}, gateway::{kamino::KaminoGateway, GatewayError, GatewayResult}, hooks::{use_boost_deposits, use_gateway, use_stake, use_wallet, BoostDeposits, Wallet}};


#[component]
pub fn PairWithdrawForm(class: Option<String>, boost_meta: BoostMeta) -> Element {
    let class = class.unwrap_or_default();
    let wallet = use_wallet();
    let withdraw_amount = use_signal::<u64>(|| 0);
    let amount_a = use_signal::<String>(|| "".to_owned());
    let amount_b = use_signal::<String>(|| "".to_owned());
    let stake = use_stake(boost_meta.lp_mint);
    let boost_deposits = use_boost_deposits(boost_meta.clone());
 
    // Build the deposit instruction
    let withdraw_ix = use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get withdraw amount
        let withdraw_amount = *withdraw_amount.read();

        // Build the instruction
        use_gateway().build_withdraw_instruction(
            boost_meta.lp_id,
            withdraw_amount,
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
                    amount_a: amount_a,
                    amount_b: amount_b,
                    withdraw_amount: withdraw_amount,
                    stake: stake,
                    boost_deposits: boost_deposits,
                }
            }

            WithdrawButton {
                enabled: if let Some(Ok(_ix)) = withdraw_ix.cloned() {
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
                    let withdraw_amount = *withdraw_amount.read();
                    if withdraw_amount == 0 {
                        return;
                    }

                    // Build ore boost withdraw instruction
                    ixs.push(
                        ore_boost_api::sdk::withdraw(
                            authority,
                            boost_meta.lp_mint,
                            withdraw_amount,
                        )
                    );

                    // Build sol ata
                    let token_a_ata = get_associated_token_address(&authority, &boost_meta.pair_mint);
                    let is_sol = boost_meta.pair_mint == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
                    if is_sol {
                        ixs.push(
                            create_associated_token_account_idempotent(&authority, &authority, &boost_meta.pair_mint, &spl_token::ID)
                        );
                        ixs.push(
                            sync_native(&spl_token::ID, &token_a_ata).unwrap()
                        );
                    }

                    // Append kamino withdraw instructions
                    let Some(Ok(withdraw_ix)) = withdraw_ix.cloned() else {
                        return;
                    };
                    ixs.push(withdraw_ix);

                    // Close the wSOL ata
                    if is_sol {
                        ixs.push(
                            close_account(&spl_token::ID, &token_a_ata, &authority, &authority, &[&authority]).unwrap()
                        );
                    }

                    // Send instructions
                    let _tx_legacy = Transaction::new_with_payer(&ixs, Some(&authority));
                    let tx = VersionedTransaction {
                        signatures: vec![],
                        message: VersionedMessage::V0(
                            v0::Message::try_compile(
                                &authority,
                                &ixs,
                                &[], // TODO LUT
                                Hash::default(),
                            ).unwrap()
                        ),
                    };
                    submit_transaction(tx);
                },
            }
        }
    }
}


#[component]
fn StakeInputs(
    mint: Pubkey,
    amount_a: Signal<String>,
    amount_b: Signal<String>,
    withdraw_amount: Signal<u64>,
    stake: Resource<GatewayResult<Stake>>,
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
                    class: "text-elements-lowEmphasis my-auto pl-1",
                    "Withdraw"
                }
                MaxButton {
                    amount_a: amount_a,
                    amount_b: amount_b,
                    stake: stake,
                    withdraw_amount: withdraw_amount,
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
                            // let Some(Ok(deposits)) = boost_deposits.cloned() else {
                            //     return;
                            // };

                            // let ratio = deposits.balance_a / deposits.balance_b;

                            // let val = e.value();
                            // if val.len().eq(&0) {
                            //     amount_a.set(val.clone());
                            //     amount_b.set(val);
                            //     return;
                            // }

                            // if let Ok(val_f64) = val.parse::<f64>() {
                            //     if val_f64 >= 0f64 {
                            //         amount_a.set(val);
                            //         amount_b.set((val_f64 / ratio).to_string());
                            //     } else {
                            //         amount_a.set("".to_string());
                            //         amount_b.set("".to_string());
                            //     }
                            // } else {
                            //     amount_a.set(val[..val.len()-1].to_string());
                            // }
                        }
                    }
                }
                Row {
                    class: "justify-between",
                    span {
                        class: "text-elements-lowEmphasis my-auto pl-1",
                        "And"
                    }
                    // MaxButton {
                    //     amount_a: amount_a,
                    //     amount_b: amount_b,
                    //     stake: stake,
                    //     withdraw_amount: withdraw_amount,
                    //     boost_deposits: boost_deposits,
                    // }
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
                            // let Some(Ok(deposits)) = boost_deposits.cloned() else {
                            //     return;
                            // };

                            // let ratio = deposits.balance_a / deposits.balance_b;

                            // let val = e.value();
                            // if val.len().eq(&0) {
                            //     amount_a.set(val.clone());
                            //     amount_b.set(val);
                            //     return;
                            // }

                            // if let Ok(val_f64) = val.parse::<f64>() {
                            //     if val_f64 >= 0f64 {
                            //         amount_a.set((val_f64 * ratio).to_string());
                            //         amount_b.set(val);
                            //     } else {
                            //         amount_a.set("".to_string());
                            //         amount_b.set("".to_string());
                            //     }
                            // } else {
                            //     amount_b.set(val[..val.len()-1].to_string());
                            // }
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
    stake: Resource<GatewayResult<Stake>>,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    withdraw_amount: Signal<u64>,
) -> Element {
    rsx! {
        button {
            class: "text-xs my-auto py-1 px-1 font-medium text-elements-lowEmphasis hover:text-elements-highEmphasis hover:cursor-pointer",
            onclick: move |_| {
                let Some(Ok(stake)) = stake.cloned() else {
                    return;
                };

                let Some(Ok(boost_deposits)) = boost_deposits.cloned() else {
                    return;
                };

                let percentage_shares = stake.balance as f64 / boost_deposits.shares as f64;
                let token_a_amount = boost_deposits.balance_a * percentage_shares;
                let token_b_amount = boost_deposits.balance_b * percentage_shares;

                let token_a_decimals = LISTED_TOKENS_BY_TICKER.get(&boost_deposits.token_a).unwrap().decimals as usize;
                let token_b_decimals = LISTED_TOKENS_BY_TICKER.get(&boost_deposits.token_b).unwrap().decimals as usize;

                amount_a.set(format!("{:.1$}", token_a_amount, token_a_decimals));
                amount_b.set(format!("{:.1$}", token_b_amount, token_b_decimals));
                withdraw_amount.set(stake.balance);
            },
            "Max"
        }
    }
}