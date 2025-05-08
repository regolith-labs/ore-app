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
pub fn TokenWithdrawForm(
    balance: Signal<GatewayResult<UiTokenAmount>>,
    stake: Signal<GatewayResult<Stake>>,
    token: Signal<Option<Token>>,
) -> Element {
    let mut input_amount = use_signal::<String>(|| "".to_owned());
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Get the stake balance
    let mut stake_balance = use_signal(|| Err(GatewayError::AccountNotFound));
    use_effect(move || {
        match stake.cloned() {
            Ok(stake) => {
                let amount_u64 = stake.balance;
                let amount_f64 = amount_to_ui_amount(amount_u64, TOKEN_DECIMALS);
                stake_balance.set(Ok(UiTokenAmount {
                    ui_amount: Some(amount_f64),
                    ui_amount_string: format!("{:.1$}", amount_f64, TOKEN_DECIMALS as usize),
                    amount: amount_u64.to_string(),
                    decimals: TOKEN_DECIMALS as u8,
                }));
            }
            _ => stake_balance.set(Err(GatewayError::Unknown)),
        };
    });

    // Build the withdraw transaction
    let tx = use_idle_withdraw_transaction(stake, input_amount, err);

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
                Fee {}
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
