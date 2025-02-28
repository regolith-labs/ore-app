use crate::{
    components::TokenInputError,
    config::Token,
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet, APP_FEE, COMPUTE_UNIT_BUFFER},
    solana::spl_token::{amount_to_ui_amount, ui_amount_to_amount},
};
use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::Stake;
use solana_program::system_instruction::transfer;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    transaction::{Transaction, VersionedTransaction},
};

const ESTIMATED_IDLE_WITHDRAW_COMPUTE_UNITS: u32 = 17195;

pub fn use_idle_withdraw_transaction(
    stake: Resource<GatewayResult<Stake>>,
    input_amount: Signal<String>,
    mut err: Signal<Option<TokenInputError>>,
    mut priority_fee: Signal<u64>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
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
        if let Some(Ok(stake)) = stake.read().as_ref() {
            if amount_to_ui_amount(stake.balance + stake.balance_pending, TOKEN_DECIMALS)
                < amount_f64
            {
                err.set(Some(TokenInputError::InsufficientBalance(Token::ore())));
                return Err(GatewayError::Unknown);
            }
        } else {
            err.set(Some(TokenInputError::InsufficientBalance(Token::ore())));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Adjust compute unit limit based on buffer -> 20,634
        let adjusted_compute_unit_limit = ESTIMATED_IDLE_WITHDRAW_COMPUTE_UNITS
            + (ESTIMATED_IDLE_WITHDRAW_COMPUTE_UNITS as f64 * COMPUTE_UNIT_BUFFER) as u32;

        log::info!(
            "adjusted_compute_unit_limit: {}",
            adjusted_compute_unit_limit
        );

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            adjusted_compute_unit_limit,
        ));

        // Build withdraw instruction
        let amount_u64 = ui_amount_to_amount(amount_f64, TOKEN_DECIMALS);
        ixs.push(ore_boost_api::sdk::withdraw(
            authority,
            MINT_ADDRESS,
            amount_u64,
        ));

        // Include ORE app fee
        ixs.push(transfer(&authority, &authority, APP_FEE));

        // Build initial transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // Get priority fee estimate
        let gateway = use_gateway();
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
            Ok(fee) => {
                log::info!("Priority fee estimate fetched: {}", fee);
                fee
            }
            Err(_) => {
                log::error!("Failed to fetch priority fee estimate");
                return Err(GatewayError::Unknown);
            }
        };

        // Add priority fee instruction
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(
            dynamic_priority_fee,
        ));

        // Set priority fee for UI
        priority_fee.set(dynamic_priority_fee);

        // Build final tx
        let tx_with_fee = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx_with_fee)
    })
}
