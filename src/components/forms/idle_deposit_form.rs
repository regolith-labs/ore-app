use dioxus::prelude::*;
use ore_boost_api::state::Stake;

use crate::gateway::{GatewayResult, UiTokenAmount};
use crate::{
    components::{Col, SubmitButton, TokenInputError, TokenInputForm},
    config::Token,
    hooks::{on_transaction_done, use_idle_deposit_transaction},
};

#[component]
pub fn IdleDepositForm(
    balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let mut input_amount = use_signal::<String>(|| "".to_owned());
    let token = use_signal(|| Some(Token::ore()));
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Build the transaction
    let tx = use_idle_deposit_transaction(stake, balance, input_amount, err);

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
                title: "Deposit".to_string(),
                balance,
                token,
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
