use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{Col, SubmitButton, TokenInputError, TokenInputForm}, config::Token, gateway::GatewayError, hooks::{on_transaction_done, use_idle_withdraw_transaction}
};
use crate::gateway::{UiTokenAmount, GatewayResult};

#[component]
pub fn IdleWithdrawForm(
    balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let mut input_amount = use_signal::<String>(|| "".to_owned());
    let err = use_signal::<Option<TokenInputError>>(|| None);
    
    // Get the stake balance
    let stake_balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let amount_u64 = stake.balance + stake.balance_pending;
        let amount_f64 = amount_to_ui_amount(amount_u64, TOKEN_DECIMALS);
        Ok(UiTokenAmount {
            ui_amount: Some(amount_f64),
            ui_amount_string: format!("{:.1$}", amount_f64, TOKEN_DECIMALS as usize),
            amount: amount_u64.to_string(),
            decimals: TOKEN_DECIMALS as u8,
        })
    });

    // Build the withdraw transaction
    let tx = use_idle_withdraw_transaction(stake, input_amount, err);

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
                token: Token::ore(),
                value: input_amount,
                update: input_amount,
                toolbar_shortcuts: true,
                err: err
            }
            SubmitButton {
                title: "Submit".to_string(),
                transaction: tx,
                err: err
            }
        }
    }
}
