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
    mut priority_fee: Signal<u64>, // Uncommented and used
    mut address_err: Signal<Option<TransferError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let gateway = use_gateway(); // Added for priority fee estimation
    use_resource(move || async move {
        err.set(None);
        address_err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            log::info!("Wallet not connected");
            return Err(GatewayError::WalletDisconnected);
        };

        // Get the selected token
        let Some(token) = selected_token.read().clone() else {
            log::info!("No token selected");
            return Err(GatewayError::NoTokenSelected);
        };

        // Validate input amount
        let amount_str = input_amount.read().clone();
        if amount_str.is_empty() {
            return Err(GatewayError::InvalidInput("Amount is empty".to_string()));
        }

        let Ok(amount_f64) = amount_str.parse::<f64>() else {
            return Err(GatewayError::InvalidInput("Amount is not a number".to_string()));
        };

        if amount_f64 <= 0f64 {
            return Err(GatewayError::InvalidInput("Amount must be greater than 0".to_string()));
        };

        // Check token balance
        let Ok(balance_data) = token_balance.read().as_ref() else {
            err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
            return Err(GatewayError::Unknown);
        };
        if balance_data.ui_amount.unwrap_or(0.0) < amount_f64 {
            err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
            return Err(GatewayError::InsufficientFunds);
        }

        // Validate destination address
        let destination_str = destination.read().clone();
        if destination_str.is_empty() {
            address_err.set(Some(TransferError::InvalidAddress));
            return Err(GatewayError::InvalidInput("Destination address is empty".to_string()));
        }

        let destination = match Pubkey::try_from(destination_str.as_str()) {
            Ok(dest) => dest,
            Err(_) => {
                address_err.set(Some(TransferError::InvalidAddress));
                return Err(GatewayError::InvalidPubkey);
            }
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

        // Create ATA idempotently
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

        // Build initial transaction to estimate priority fee
        let temp_tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // Get priority fee estimate
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&temp_tx).await {
            Ok(fee) => fee,
            Err(e) => {
                log::warn!(
                    "Failed to fetch priority fee estimate: {:?}, using fallback {}",
                    e,
                    *priority_fee.read()
                );
                *priority_fee.read() // Fallback to signal value
            }
        };

        // Add priority fee instruction at the start
        ixs.insert(
            0,
            ComputeBudgetInstruction::set_compute_unit_price(dynamic_priority_fee),
        );

        // Calculate priority fee in lamports for UI
        let adjusted_compute_unit_limit_u64: u64 = COMPUTE_UNIT_LIMIT.into();
        let dynamic_priority_fee_in_lamports =
            (dynamic_priority_fee * adjusted_compute_unit_limit_u64) / 1_000_000;
        priority_fee.set(dynamic_priority_fee_in_lamports);

        // Build final transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}