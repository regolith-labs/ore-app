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
    hooks::{use_wallet, Wallet, APP_FEE, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
    pages::TransferError,
    solana::{
        spl_associated_token_account,
        spl_token::{self, instruction::transfer as spl_transfer, ui_amount_to_amount},
    },
};

pub fn use_transfer_transaction(
    destination: Signal<String>,
    selected_token: Signal<Option<Token>>,
    input_amount: Signal<String>,
    token_balance: Signal<GatewayResult<UiTokenAmount>>,
    mut err: Signal<Option<TokenInputError>>,
    // priority_fee: Signal<u64>,
    mut address_err: Signal<Option<TransferError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        err.set(None);
        address_err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            log::info!("wallet");
            return Err(GatewayError::WalletDisconnected);
        };

        // Get the selected token
        let Some(token) = selected_token.cloned() else {
            log::info!("select token");
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

        // If amount is less than the token balance, disable
        let balance = token_balance.cloned();
        if let Ok(balance_data) = balance {
            if balance_data.ui_amount.unwrap_or(0.0) < amount_f64 {
                err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
                return Err(GatewayError::Unknown);
            }
        } else {
            // User might not have a token balance to begin with
            err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
            return Err(GatewayError::Unknown);
        }

        // Check if address is empty
        let destination_str = destination.cloned();
        if destination_str.is_empty() {
            return Err(GatewayError::Unknown);
        }

        // Check if Pubkey is valid
        let destination = if let Ok(dest) = Pubkey::try_from(destination_str.as_str()) {
            dest
        } else {
            address_err.set(Some(TransferError::InvalidAddress));
            return Err(GatewayError::Unknown);
        };

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Convert to u64
        let amount_u64 = ui_amount_to_amount(amount_f64, token.decimals);

        // Obtain ATAs
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
        if token.mint == Token::sol().mint {
            ixs.push(transfer(&authority, &destination, amount_u64));
        } else {
            ixs.push(spl_transfer(
                &spl_token::ID,
                &from_ata,
                &to_ata,
                &authority,
                &[],
                amount_u64,
            )?);
        }

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, APP_FEE));

        // Build final tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
