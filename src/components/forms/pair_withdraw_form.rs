use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::{
    components::{Col, SubmitButton, TokenInputError}, 
    config::BoostMeta, 
    gateway::{GatewayError, GatewayResult, UiTokenAmount}, 
    hooks::{on_transaction_done, use_pair_withdraw_transaction, BoostDeposits}
};
use super::token_input_form::*;



#[component]
pub fn PairWithdrawForm(
    class: Option<String>,
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>
) -> Element {
    let class = class.unwrap_or_default();
    let mut input_amount_a = use_signal::<String>(|| "".to_owned());
    let mut input_amount_b = use_signal::<String>(|| "".to_owned());
    let err = use_signal::<Option<TokenInputError>>(|| None);
 
    // Refresh data, if transaction success
    on_transaction_done(move |_sig| {
        boost_deposits.restart();
        token_a_balance.restart();
        token_b_balance.restart();
        lp_balance.restart();
        stake.restart();
    });

    // Build pair deposit transaction
    let tx = use_pair_withdraw_transaction(
        boost_meta, 
        boost_deposits, 
        stake, 
        input_amount_a, 
        input_amount_b, 
        err
    );
    
    // Get tokens
    let (token_a, token_b) = if let Some(Ok(boost_deposits)) = boost_deposits.cloned() {
        (Some(boost_deposits.token_a), Some(boost_deposits.token_b))
    } else {
        (None, None)
    };

    // Get stake balances
    let stake_a_balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(boost_deposits)) = boost_deposits.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let percentage_shares = stake.balance as f64 / boost_deposits.shares as f64;
        let token_a_amount = boost_deposits.balance_a_f64 * percentage_shares;
        let token_a_decimals = boost_deposits.token_a.decimals as usize;
        Ok(UiTokenAmount {
            ui_amount: Some(token_a_amount),
            ui_amount_string: format!("{:.1$}", token_a_amount, token_a_decimals),
            amount: token_a_amount.to_string(), // TODO Convert to u64
            decimals: token_a_decimals as u8,
        })
    });
    let stake_b_balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(boost_deposits)) = boost_deposits.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let percentage_shares = stake.balance as f64 / boost_deposits.shares as f64;
        let token_b_amount = boost_deposits.balance_b_f64 * percentage_shares;
        let token_b_decimals = boost_deposits.token_b.decimals as usize;
        Ok(UiTokenAmount {
            ui_amount: Some(token_b_amount),
            ui_amount_string: format!("{:.1$}", token_b_amount, token_b_decimals),
            amount: token_b_amount.to_string(), // TODO Convert to u64
            decimals: token_b_decimals as u8,
        })
    });

    let process_input = move |val: String, prior_val: String, flag: bool| {
         // Define function to safely update input values
         let mut safe_update = move |new_val: String| {
            let new_val_f64 = new_val.parse::<f64>().unwrap_or(0.0);
            let prior_val_f64 = prior_val.parse::<f64>().unwrap_or(0.0);
            if new_val_f64 != prior_val_f64 {
                if flag {
                    input_amount_b.set(new_val);
                } else {
                    input_amount_a.set(new_val);
                }
            }
        };

        // Get resources
        let Some(Ok(stake)) = stake.cloned() else {
            return;
        };
        let Some(Ok(boost_deposits)) = boost_deposits.cloned() else {
            return;
        };

        // Parse input value
        let val_f64 = val.parse::<f64>().unwrap_or(0.0);
        if val_f64 == 0.0 {
            safe_update(val);
            return;
        }

        // Calculate percentage shares
        let percentage_shares = stake.balance as f64 / boost_deposits.shares as f64;
        let token_a_shares = boost_deposits.balance_a_f64 * percentage_shares;
        let token_b_shares = boost_deposits.balance_b_f64 * percentage_shares;

        // Update input values
        if flag {
            let percent_to_withdraw = val_f64 / token_a_shares;
            safe_update(
                format!("{:.1$}", 
                token_b_shares * percent_to_withdraw, 
                boost_deposits.token_b.decimals as usize)
            );
        } else {
            let percent_to_withdraw = val_f64 / token_b_shares;
            safe_update(
                format!("{:.1$}", 
                token_a_shares * percent_to_withdraw, 
                boost_deposits.token_a.decimals as usize)
            );
        }
    };

    // Process input stream a
    let b = input_amount_b.cloned();
    use_effect(move || {
        process_input(input_amount_a.read().clone(), b.clone(), true);
    });

    // Process input stream b
    let a = input_amount_a.cloned();
    use_effect(move || {
        process_input(input_amount_b.read().clone(), a.clone(), false);
    });

    rsx! {
        Col {
            gap: 4,
            Col {
                class: "w-full p-4 lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0 {class}",
                gap: 4,
                TokenInputForm {
                    title: "Withdraw".to_string(),
                    balance: stake_a_balance,
                    token: token_a,
                    value: input_amount_a,
                    toolbar_shortcuts: true,
                    err: err
                }
                TokenInputForm {
                    title: "And".to_string(),
                    balance: stake_b_balance,
                    token: token_b,
                    value: input_amount_b,
                    err: err
                }
            }
            SubmitButton {
                title: "Submit".to_string(),
                transaction: tx,
                err: err
            }
        }
    }
}
