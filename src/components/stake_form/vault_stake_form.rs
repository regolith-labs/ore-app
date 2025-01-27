use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::{spl_associated_token_account, spl_token::{amount_to_ui_amount, ui_amount_to_amount}};
use solana_sdk::transaction::Transaction;

use crate::{
    components::{submit_transaction, Col, Row, WalletIcon}, hooks::{use_ore_balance, use_stake, use_wallet, Wallet}
};
use crate::gateway::{ui_token_amount::UiTokenAmount, GatewayResult};
use super::common::*;

#[component]
pub fn VaultStakeForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| StakeTab::Deposit);
    let ore_balance = use_ore_balance();
    let ore_stake = use_stake(ore_api::consts::MINT_ADDRESS);

    rsx! {
        Col {
            class: "{class}",
            StakeTabs {
                tab: tab
            }
            match *tab.read() {
                StakeTab::Deposit => rsx! {
                    VaultDepositForm {
                        ore_balance: ore_balance,
                        ore_stake: ore_stake,
                        
                    }
                },
                StakeTab::Withdraw => rsx! {
                    VaultWithdrawForm {
                        ore_balance: ore_balance,
                        ore_stake: ore_stake,
                    }
                }
            }
        }
    }
}

#[component]
fn VaultDepositForm(
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let wallet = use_wallet();
    let deposit_amount = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);

    // Build the transaction
    use_effect(move || {
        // Get wallet address
        let Wallet::Connected(_authority) = *wallet.read() else {
            enabled.set(false);
            return;
        };

         // If empty, disable
         let amount_str = deposit_amount.cloned();
         if amount_str.is_empty() {
            enabled.set(false);
            return;
        }

        // If input isn't a number, disable
        let Ok(amount_f64) = amount_str.parse::<f64>() else {
            enabled.set(false);
            return;
        };

        // If amount is 0, disable
        if amount_f64 == 0f64 {
            enabled.set(false);
            return;
        }

        // If amount is greater than ore balance, disable
        if let Some(Ok(ore_balance)) = ore_balance.read().as_ref() {
            if ore_balance.ui_amount.unwrap_or(0.0) < amount_f64 {
                enabled.set(false);
                return;
            }
        } else {
            enabled.set(false);
            return;
        }

        enabled.set(true);
    });

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
                VaultInput {
                    tab: StakeTab::Deposit,
                    input_amount: deposit_amount,
                    ore_balance: ore_balance,
                    ore_stake: ore_stake
                }
            }
            // StakeDetails {}
            SubmitButton {
                enabled: enabled,
                onclick: move |_| {
                    let mut ixs = vec![];
                    let Wallet::Connected(authority) = *wallet.read() else {
                        return;
                    };
                    if let Some(Ok(_)) = *ore_stake.read() {
                        // Do nothing
                    } else {
                        ixs.push(ore_boost_api::sdk::open(authority, authority, MINT_ADDRESS));
                    }
                    let amount_f64 = deposit_amount.cloned().parse::<f64>().unwrap();
                    let amount_u64 = ui_amount_to_amount(amount_f64, TOKEN_DECIMALS);
                    ixs.push(ore_boost_api::sdk::deposit(authority, MINT_ADDRESS, amount_u64));
                    let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
                    submit_transaction(tx);
                }
            }
        }
    }
}

#[component]
fn VaultInput(
    tab: StakeTab,
    input_amount: Signal<String>,
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let title = match tab {
        StakeTab::Deposit => "Deposit".to_string(),
        StakeTab::Withdraw => "Withdraw".to_string(),
    };
    rsx! {
        Col {
            class: "w-full p-4",
            gap: 2,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-midEmphasis my-auto pl-1",
                    "{title}"
                }
                MaxButton {
                    tab: tab,
                    input_amount: input_amount,
                    ore_balance: ore_balance,
                    ore_stake: ore_stake
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
                    value: input_amount.cloned(),
                    oninput: move |e| {
                        let s = e.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            input_amount.set(s);
                        } else {
                            input_amount.set(s[..s.len()-1].to_string());
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VaultWithdrawForm(
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let wallet = use_wallet();
    let withdraw_amount = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);

    // Build the transaction
    use_effect(move || {
        // Get wallet address
        let Wallet::Connected(_authority) = *wallet.read() else {
            enabled.set(false);
            return;
        };

        // If empty, disable
        let amount_str = withdraw_amount.cloned();
        if amount_str.is_empty() {
            enabled.set(false);
            return;
        }

        // If input isn't a number, disable
        let Ok(amount) = amount_str.parse::<f64>() else {
            enabled.set(false);
            return;
        };

        // If amount is 0, disable
        if amount == 0f64 {
            enabled.set(false);
            return;
        }

        // If amount is greater than stake balance, disable
        if let Some(Ok(stake)) = ore_stake.read().as_ref() {
            if amount_to_ui_amount(stake.balance + stake.balance_pending, TOKEN_DECIMALS) < amount {
                enabled.set(false);
                return;
            }
        } else {
            enabled.set(false);
            return;
        }

        enabled.set(true);
    });

    // let balance = use_token_balance(mint);
    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
                VaultInput {
                    tab: StakeTab::Withdraw,
                    input_amount: withdraw_amount,
                    ore_balance: ore_balance,
                    ore_stake: ore_stake
                }
            }
            // StakeDetails {}
            SubmitButton {
                enabled: enabled,
                onclick: move |_| {
                    let mut ixs = vec![];
                    let Wallet::Connected(authority) = *wallet.read() else {
                        return;
                    };
                    let x = spl_associated_token_account::get_associated_token_address(&authority, &MINT_ADDRESS);
                    log::info!("x: {:?}", x);
                    let amount_f64 = withdraw_amount.cloned().parse::<f64>().unwrap();
                    let amount_u64 = ui_amount_to_amount(amount_f64, TOKEN_DECIMALS);
                    ixs.push(ore_boost_api::sdk::withdraw(authority, MINT_ADDRESS, amount_u64));
                    let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
                    submit_transaction(tx);
                }
            }
        }
    }
}

#[component]
fn MaxButton(
    tab: StakeTab,
    mut input_amount: Signal<String>, 
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>
) -> Element {
    let balance = match tab {
        StakeTab::Deposit => {
            if let Some(Ok(balance)) = ore_balance.cloned() {
                balance.ui_amount.unwrap_or(0.0)
            } else {
                0.0
            }
        }
        StakeTab::Withdraw => {
            if let Some(Ok(stake)) = ore_stake.cloned() {
                amount_to_ui_amount(stake.balance + stake.balance_pending, TOKEN_DECIMALS)
            } else {
                0.0
            }
        }
    };

    rsx! {
        button {
            class: "flex flex-row gap-2 py-1 px-1 text-elements-lowEmphasis hover:cursor-pointer hover:text-elements-highEmphasis my-auto",
            onclick: move |_| {
                input_amount.set(balance.to_string());
            },
            WalletIcon { 
                class: "h-4 my-auto" 
            }
            span { 
                class: "my-auto text-xs font-medium", 
                "{balance}" 
            }
        }
    }
}
