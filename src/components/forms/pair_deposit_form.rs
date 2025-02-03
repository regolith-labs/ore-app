use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::{
    components::{Col, SubmitButton, TokenInputError}, 
    config::BoostMeta, 
    gateway::{GatewayResult, UiTokenAmount}, 
    hooks::{on_transaction_done, use_pair_deposit_transaction, BoostDeposits}
};
use super::token_input_form::*;


#[component]
pub fn PairDepositForm(
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
    let tx = use_pair_deposit_transaction(
        boost_meta, 
        boost_deposits, 
        lp_balance, 
        stake, 
        token_a_balance, 
        token_b_balance, 
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

    // Update input values based on updates from the form
    let mut process_input_stream = move |val: String, flag: bool| {
        // Parse event value
        if val.len().eq(&0) {
            input_amount_a.set(val.clone());
            input_amount_b.set(val.clone());
            return;
        }

        // Get resources
        let Some(Ok(deposits)) = boost_deposits.cloned() else {
            return;
        };
        let Some(Ok(token_a_balance)) = token_a_balance.cloned() else {
            return;
        };
        let Some(Ok(token_b_balance)) = token_b_balance.cloned() else {
            return;
        };

        // Calculate deposit ratio
        let ratio = deposits.balance_a_f64 / deposits.balance_b_f64;

        // Update input values
        if let Ok(val_f64) = val.parse::<f64>() {
            if val_f64 >= 0f64 {
                if flag {
                    input_amount_a.set(val.clone());
                    input_amount_b.set(
                        format!("{:.1$}", (val_f64 / ratio), token_b_balance.decimals as usize)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
                    );
                } else {
                    input_amount_b.set(val.clone());
                    input_amount_a.set(
                        format!("{:.1$}", (val_f64 * ratio), token_a_balance.decimals as usize)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
                    );
                }
            } else {
                input_amount_a.set("0".to_string());
                input_amount_b.set("0".to_string());
            }
        }
    };

    // Process input streams
    use_effect(move || {
        process_input_stream(input_stream_a.read().clone(), true);
    });
    use_effect(move || {
        process_input_stream(input_stream_b.read().clone(), false);
    });

    rsx! {
        Col {
            gap: 4,
            Col {
                class: "w-full p-4 lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0 {class}",
                gap: 4,
                TokenInputForm {
                    title: "Deposit".to_string(),
                    balance: token_a_balance,
                    token: token_a,
                    value: input_amount_a,
                    update: input_stream_a,
                    toolbar_shortcuts: true,
                    err: err
                }
                TokenInputForm {
                    title: "And".to_string(),
                    balance: token_b_balance,
                    token: token_b,
                    value: input_amount_b,
                    update: input_stream_b,
                    toolbar_shortcuts: true,
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

