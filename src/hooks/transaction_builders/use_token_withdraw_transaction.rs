use crate::{
    components::TokenInputError,
    config::Token,
    gateway::{GatewayError, GatewayResult},
    hooks::{use_wallet, Wallet, APP_FEE, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
    solana::spl_token::{amount_to_ui_amount, ui_amount_to_amount},
};
use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

#[cfg(not(feature = "web"))]
use super::tip_ix;

pub fn use_token_withdraw_transaction(
    stake: Signal<GatewayResult<Stake>>,
    input_amount: Signal<String>,
    token: Signal<Option<Token>>,
    mut err: Signal<Option<TokenInputError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get token
        let Some(token) = token.cloned() else {
            return Err(GatewayError::Unknown);
        };

        // If empty, disable
        let amount_str = input_amount.cloned();
        if amount_str.is_empty() {
            return Err(GatewayError::Unknown);
        }

        // If input isn't a number, disable
        let Ok(amount_f64) = amount_str.parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };

        // If amount is 0, disable
        if amount_f64 == 0f64 {
            return Err(GatewayError::Unknown);
        }

        // If amount is greater than stake balance, disable
        if let Ok(stake) = stake.read().as_ref() {
            if amount_to_ui_amount(stake.balance, token.decimals) < amount_f64 {
                err.set(Some(TokenInputError::InsufficientBalance(token)));
                return Err(GatewayError::Unknown);
            }
        } else {
            err.set(Some(TokenInputError::InsufficientBalance(token)));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Build withdraw instruction
        let amount_u64 = ui_amount_to_amount(amount_f64, token.decimals);
        ixs.push(ore_boost_api::sdk::withdraw(
            authority, token.mint, amount_u64,
        ));

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, APP_FEE));

        #[cfg(not(feature = "web"))]
        // Add jito tip
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
