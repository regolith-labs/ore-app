use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::{
    components::{Col, SubmitButton, TokenInputError}, 
    config::BoostMeta, 
    gateway::{GatewayResult, UiTokenAmount}, 
    hooks::{on_transaction_done, use_pair_withdraw_transaction, use_stake_balances},
    utils::LiquidityPair
};
use super::token_input_form::*;



#[component]
pub fn PairWithdrawForm(
    class: Option<String>,
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>
) -> Element {
    let class = class.unwrap_or_default();
    let mut input_amount_a = use_signal::<String>(|| "".to_owned());
    let mut input_amount_b = use_signal::<String>(|| "".to_owned());
    let mut input_stream_a = use_signal::<String>(|| "".to_owned());
    let mut input_stream_b = use_signal::<String>(|| "".to_owned());
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Get tokens
    let (token_a, token_b) = if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
        (Some(liquidity_pair.token_a), Some(liquidity_pair.token_b))
    } else {
        (None, None)
    };

    // Get stake balances
    let (stake_a_balance, stake_b_balance) = use_stake_balances(liquidity_pair, stake);

    // Build pair deposit transaction
    let tx = use_pair_withdraw_transaction(
        boost_meta, 
        liquidity_pair, 
        stake,
        stake_a_balance,
        stake_b_balance,
        input_amount_a, 
        input_amount_b, 
        err
    );

    // Refresh data, if transaction success
    on_transaction_done(move |_sig| {
        input_stream_a.set("".to_owned());
        input_stream_b.set("".to_owned());
    });

    // Update input values based on updates from the form
    let mut process_input_stream = move |val: String, flag: bool| {
        // Empty input
        if val.len().eq(&0) {
            input_amount_a.set(val.clone());
            input_amount_b.set(val.clone());
            return;
        }

        // Parse input value
        let val_f64 = val.parse::<f64>().unwrap_or(0.0);
        if val_f64 == 0.0 {
            if flag {
                input_amount_a.set(val.clone());
                input_amount_b.set("0".to_string());
            } else {
                input_amount_b.set(val.clone());
                input_amount_a.set("0".to_string());
            }
            return;
        }

        // Get resources
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return;
        };

        // Calculate percentage shares
        let percentage_shares = stake.balance as f64 / liquidity_pair.shares as f64;
        let token_a_shares = liquidity_pair.balance_a_f64 * percentage_shares;
        let token_b_shares = liquidity_pair.balance_b_f64 * percentage_shares;

        // Update input values
        if flag {
            let percent_to_withdraw = val_f64 / token_a_shares;
            input_amount_a.set(val.clone());
            input_amount_b.set(
                format!("{:.1$}", token_b_shares * percent_to_withdraw, liquidity_pair.token_b.decimals as usize)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            );
        } else {
            let percent_to_withdraw = val_f64 / token_b_shares;
            input_amount_b.set(val.clone());
            input_amount_a.set(
                format!("{:.1$}", token_a_shares * percent_to_withdraw, liquidity_pair.token_a.decimals as usize)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            );
        }
    };

    // Process input streams
    use_effect(move || {
        process_input_stream(input_stream_a.cloned(),  true);
    });
    use_effect(move || {
        process_input_stream(input_stream_b.cloned(), false);
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
