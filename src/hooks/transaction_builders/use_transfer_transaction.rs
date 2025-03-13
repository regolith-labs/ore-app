use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    components::TokenInputError,
    config::Token,
    gateway::{GatewayError, GatewayResult, UiTokenAmount},
    hooks::{use_gateway, use_wallet, Wallet, APP_FEE, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
    solana::{
        spl_associated_token_account,
        spl_token::{self, instruction::transfer as spl_transfer, ui_amount_to_amount},
    },
};

pub fn use_transfer_transaction(
    destination: Signal<String>,
    selected_token: Signal<Option<Token>>,
    input_amount: Signal<String>,
    token_balance: Resource<GatewayResult<UiTokenAmount>>,
    mut err: Signal<Option<TokenInputError>>,
    mut priority_fee: Signal<u64>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get the destination pubkey
        let destination_str = destination.read().clone();
        if destination_str.is_empty() {
            return Err(GatewayError::Unknown);
        }

        let Ok(destination) = Pubkey::try_from(destination_str.as_str()) else {
            // Just return an error without setting a specific TokenInputError
            return Err(GatewayError::Unknown);
        };

        // Get the selected token
        let Some(token) = selected_token.read().clone() else {
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

        // If amount is greater than token balance, disable
        if let Some(Ok(balance)) = token_balance.read().as_ref() {
            if balance.ui_amount.unwrap_or(0.0) < amount_f64 {
                err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
                return Err(GatewayError::Unknown);
            }
        } else {
            err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // For SPL tokens, use token transfer
        let amount_u64 = ui_amount_to_amount(amount_f64, token.decimals);
        let from_ata =
            spl_associated_token_account::get_associated_token_address(&authority, &token.mint);
        let to_ata =
            spl_associated_token_account::get_associated_token_address(&destination, &token.mint);

        // Always create the token account idempotently (will not fail if it already exists)
        ixs.push(
            spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                &authority,
                &destination,
                &token.mint,
                &spl_token::ID,
            ),
        );

        // Add transfer instruction
        ixs.push(spl_transfer(
            &spl_token::ID,
            &from_ata,
            &to_ata,
            &authority,
            &[],
            amount_u64,
        )?);

        // // Include ORE app fee
        // let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        // ixs.push(transfer(&authority, &app_fee_account, APP_FEE));

        // // Build initial transaction to estimate priority fee
        // let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // // Get priority fee estimate
        // let gateway = use_gateway();
        // let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
        //     Ok(fee) => fee,
        //     Err(_) => {
        //         log::error!("Failed to fetch priority fee estimate");
        //         return Err(GatewayError::Unknown);
        //     }
        // };

        // // Add priority fee instruction
        // ixs.insert(
        //     1,
        //     ComputeBudgetInstruction::set_compute_unit_price(dynamic_priority_fee),
        // );

        // // Calculate priority fee in lamports
        // let adjusted_compute_unit_limit_u64: u64 = COMPUTE_UNIT_LIMIT.into();
        // let dynamic_priority_fee_in_lamports =
        //     (dynamic_priority_fee * adjusted_compute_unit_limit_u64) / 1_000_000;

        // // Set priority fee for UI
        // priority_fee.set(dynamic_priority_fee_in_lamports);

        // Build final tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
