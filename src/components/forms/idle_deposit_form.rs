use crate::{
    components::{Col, Fee, SubmitButton, TokenInputError, TokenInputForm},
    config::Token,
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{on_transaction_done, use_idle_deposit_transaction},
};
use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use ore_types::request::TransactionType;

#[component]
pub fn IdleDepositForm(
    balance: Signal<GatewayResult<UiTokenAmount>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Element {
    let mut input_amount = use_signal::<String>(|| "".to_owned());
    let token = use_signal(|| Some(Token::ore()));
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Build the transaction
    let tx = use_idle_deposit_transaction(stake, balance, input_amount, err);

    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
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
            Col {
                class: "w-full px-4",
                Fee {}
            }
            SubmitButton {
                title: "Submit".to_string(),
                transaction: tx,
                err: err,
                tx_type: TransactionType::BoostDeposit
            }
        }
    }
}
