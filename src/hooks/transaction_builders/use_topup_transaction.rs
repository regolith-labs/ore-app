use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    native_token::sol_to_lamports,
    pubkey::{ParsePubkeyError, Pubkey},
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    components::TokenInputError,
    config::Token,
    gateway::{GatewayError, GatewayResult, UiTokenAmount},
    hooks::{use_wallet, Wallet, APP_FEE, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
};

#[cfg(not(feature = "web"))]
use super::tip_ix;

pub fn use_topup_transaction(
    destination: Memo<Result<Pubkey, ParsePubkeyError>>,
    input_amount: Signal<String>,
    sol_balance: Signal<GatewayResult<UiTokenAmount>>,
    mut err: Signal<Option<TokenInputError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get the destination pubkey
        let Ok(destination) = destination.cloned() else {
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

        // If amount is greater than SOL balance, disable
        if let Ok(sol_balance) = sol_balance.read().as_ref() {
            if sol_balance.ui_amount.unwrap_or(0.0) < amount_f64 {
                err.set(Some(TokenInputError::InsufficientBalance(Token::sol())));
                return Err(GatewayError::Unknown);
            }
        } else {
            err.set(Some(TokenInputError::InsufficientBalance(Token::sol())));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Build transfer instruction
        let lamports = sol_to_lamports(amount_f64);
        ixs.push(transfer(&authority, &destination, lamports));

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, APP_FEE));

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

        #[cfg(not(feature = "web"))]
        // Add jito tip
        ixs.push(tip_ix(&authority));

        // Build final tx with priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
