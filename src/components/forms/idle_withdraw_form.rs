use crate::{
    components::{Col, Fee, SubmitButton, TokenInputError, TokenInputForm},
    config::Token,
    gateway::{GatewayError, GatewayResult, UiTokenAmount},
    hooks::{on_transaction_done, use_idle_withdraw_transaction},
    solana::spl_token::amount_to_ui_amount,
};
use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::Stake;
use ore_types::request::TransactionType;

#[component]
pub fn IdleWithdrawForm(
    balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let mut input_amount = use_signal::<String>(|| "".to_owned());
    let token = use_signal(|| Some(Token::ore()));
    let err = use_signal::<Option<TokenInputError>>(|| None);
    let priority_fee = use_signal::<u64>(|| 0);

    // Get the stake balance
    let stake_balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let amount_u64 = stake.balance;
        let amount_f64 = amount_to_ui_amount(amount_u64, TOKEN_DECIMALS);
        Ok(UiTokenAmount {
            ui_amount: Some(amount_f64),
            ui_amount_string: format!("{:.1$}", amount_f64, TOKEN_DECIMALS as usize),
            amount: amount_u64.to_string(),
            decimals: TOKEN_DECIMALS as u8,
        })
    });

    // Build the withdraw transaction
    let tx = use_idle_withdraw_transaction(stake, input_amount, err, priority_fee);

    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        balance.restart();
        stake.restart();
        input_amount.set("".to_owned());
    });

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            TokenInputForm {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0 p-4",
                title: "Withdraw".to_string(),
                balance: stake_balance,
                token,
                value: input_amount,
                update: input_amount,
                toolbar_shortcuts: true,
                err: err
            }
            Col {
                class: "w-full px-4",
                Fee { priority_fee: priority_fee.clone() }
            }
            SubmitButton {
                title: "Submit".to_string(),
                transaction: tx,
                err: err,
                tx_type: TransactionType::BoostWithdraw
            }
        }
    }
}
