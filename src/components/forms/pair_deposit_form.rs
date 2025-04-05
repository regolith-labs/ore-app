use crate::{
    components::{Col, ConfirmationDialog, Fee, SubmitButton, TokenInputError, TokenInputForm},
    config::BoostMeta,
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{on_transaction_done, use_pair_deposit_transaction},
    utils::LiquidityPair,
};
use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use ore_types::request::TransactionType;

#[component]
pub fn PairDepositForm(
    class: Option<String>,
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Signal<GatewayResult<Stake>>,
    token_a_balance: Signal<GatewayResult<UiTokenAmount>>,
    token_b_balance: Signal<GatewayResult<UiTokenAmount>>,
) -> Element {
    let class = class.unwrap_or_default();
    let mut token_a = use_signal(|| None);
    let mut token_b = use_signal(|| None);
    let mut input_amount_a = use_signal::<String>(|| "".to_owned());
    let mut input_amount_b = use_signal::<String>(|| "".to_owned());
    let mut input_stream_a = use_signal::<String>(|| "".to_owned());
    let mut input_stream_b = use_signal::<String>(|| "".to_owned());
    let mut err = use_signal::<Option<TokenInputError>>(|| None);

    // Refresh data, if transaction success
    on_transaction_done(move |_sig| {
        input_stream_a.set("".to_owned());
        input_stream_b.set("".to_owned());
    });

    // Build pair deposit transaction
    let tx = use_pair_deposit_transaction(
        boost_meta,
        liquidity_pair,
        lp_balance,
        stake,
        token_a_balance,
        token_b_balance,
        input_amount_a,
        input_amount_b,
        err,
    );

    // Get tokens
    use_effect(move || {
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            token_a.set(None);
            token_b.set(None);
            return;
        };
        token_a.set(Some(liquidity_pair.token_a));
        token_b.set(Some(liquidity_pair.token_b));
    });

    // Update input values based on updates from the form
    let mut process_input_stream = move |val: String, flag: bool| {
        // Parse event value
        if val.len().eq(&0) {
            err.set(None);
            input_amount_a.set(val.clone());
            input_amount_b.set(val.clone());
            return;
        }

        // Get resources
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return;
        };

        // Calculate deposit ratio
        let ratio = liquidity_pair.balance_a_f64 / liquidity_pair.balance_b_f64;

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

        // Update input values
        if flag {
            input_amount_a.set(val.clone());
            input_amount_b.set(
                format!(
                    "{:.1$}",
                    (val_f64 / ratio),
                    liquidity_pair.token_b.decimals as usize
                )
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
            );
        } else {
            input_amount_b.set(val.clone());
            input_amount_a.set(
                format!(
                    "{:.1$}",
                    (val_f64 * ratio),
                    liquidity_pair.token_a.decimals as usize
                )
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
            );
        }
    };

    // Process input streams
    use_effect(move || {
        process_input_stream(input_stream_a.cloned(), true);
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
            Col {
                class: "w-full px-4",
                Fee {},
            }

            SubmitButton {
                title: "Submit".to_string(),
                transaction: tx,
                err: err,
                tx_type: TransactionType::BoostDeposit,
                confirmation: ConfirmationDialog {
                    title: "Financial risks".to_string(),
                    detail: "Providing liquidity comes with inherent financial risk, including but not limited to, divergence loss.\nDivergence loss can occur when the relative price of the deposited tokens changes and the value of the deposit becomes less compared to holding the tokens separately.\nOnce deposited, your exposure to each token can change.".to_string(),
                    ack: "I acknowledge the risks, and I alone am responsible for my financial decisions".to_string(),
                },
            }
        }
    }
}
