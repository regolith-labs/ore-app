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
        err.set(None);
        log::info!("IN resource");
        log::info!("input amount: {:?}", input_amount);
        log::info!("token balance: {:?}", token_balance.cloned());

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            log::info!("wallet");
            return Err(GatewayError::WalletDisconnected);
        };

        // Get the selected token
        log::info!("before token");
        let Some(token) = selected_token.cloned() else {
            log::info!("select token");
            return Err(GatewayError::Unknown);
        };
        log::info!("after token");

        // If empty, disable
        log::info!("before string");
        let amount_str = input_amount.cloned();
        if amount_str.is_empty() {
            log::info!("empty string");
            return Err(GatewayError::Unknown);
        }
        log::info!("after string");

        // If input isn't a number, disable
        log::info!("before parse");
        let Ok(amount_f64) = amount_str.parse::<f64>() else {
            log::info!("nan");
            return Err(GatewayError::Unknown);
        };
        log::info!("after parse");

        // If amount is 0, disable
        log::info!("before 0");
        if amount_f64 == 0f64 {
            log::info!("amount 0");
            return Err(GatewayError::Unknown);
        }
        log::info!("after 0");

        // If amount is less than the token balance, disable
        log::info!("before balance");
        if let Some(Ok(balance)) = token_balance.cloned() {
            log::info!("outer balance");
            if balance.ui_amount.unwrap_or(0.0) < amount_f64 {
                log::info!("INSUFFICIENT BALANCE");
                err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
                return Err(GatewayError::Unknown);
            }
        } else {
            // User might not have a token balance to begin with
            log::info!("IN ELSE");
            err.set(Some(TokenInputError::InsufficientBalance(token.clone())));
            return Err(GatewayError::Unknown);
        }

        // Get the destination pubkey
        let destination_str = destination.cloned();
        if destination_str.is_empty() {
            return Err(GatewayError::Unknown);
        }

        // Check if address is valid
        let Ok(destination) = Pubkey::try_from(destination_str.as_str()) else {
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
