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
    let input_stream_a = use_signal::<String>(|| "".to_owned());
    let input_stream_b = use_signal::<String>(|| "".to_owned());
    let err = use_signal::<Option<TokenInputError>>(|| None);
 
    // Refresh data, if transaction success
    on_transaction_done(move |_sig| {
        boost_deposits.restart();
        token_a_balance.restart();
        token_b_balance.restart();
        lp_balance.restart();
        stake.restart();
        input_amount_a.set("".to_owned());
        input_amount_b.set("".to_owned());
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

    // Update input values based on updates from the form
    let mut process_input_stream = move |val: String, flag: bool| {
        // Empty input
        if val.len().eq(&0) {
            input_amount_a.set(val.clone());
            input_amount_b.set(val.clone());
            return;
        }

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
            input_amount_a.set("0".to_string());
            input_amount_b.set("0".to_string());
            return;
        }

        // Calculate percentage shares
        let percentage_shares = stake.balance as f64 / boost_deposits.shares as f64;
        let token_a_shares = boost_deposits.balance_a_f64 * percentage_shares;
        let token_b_shares = boost_deposits.balance_b_f64 * percentage_shares;

        // Update input values
        if flag {
            let percent_to_withdraw = val_f64 / token_a_shares;
            input_amount_a.set(val.clone());
            input_amount_b.set(
                format!("{:.1$}", token_b_shares * percent_to_withdraw, boost_deposits.token_b.decimals as usize)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            );
        } else {
            let percent_to_withdraw = val_f64 / token_b_shares;
            input_amount_b.set(val.clone());
            input_amount_a.set(
                format!("{:.1$}", token_a_shares * percent_to_withdraw, boost_deposits.token_a.decimals as usize)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            );
        }
    };

    // Process input streams
    use_effect(move || {
        process_input_stream(input_amount_a.read().clone(),  true);
    });
    use_effect(move || {
        process_input_stream(input_amount_b.read().clone(), false);
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
                    update: input_stream_a,
                    toolbar_shortcuts: true,
                    err: err
                }
                TokenInputForm {
                    title: "And".to_string(),
                    balance: stake_b_balance,
                    token: token_b,
                    value: input_amount_b,
                    update: input_stream_b,
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
