use std::str::FromStr;

use crate::config::Token;
use crate::gateway::jupiter::JupiterGateway;
use crate::hooks::{use_gateway, use_wallet, GetPubkey};
use crate::{
    components::TokenInputError,
    gateway::{solana::SolanaGateway, GatewayError, GatewayResult, UiTokenAmount},
    solana::spl_associated_token_account::{
        get_associated_token_address, get_or_create_associated_token_address,
    },
};
use dioxus::prelude::*;
use jupiter_dca_sdk::{
    accounts::Dca,
    instructions::{OpenDcaV2, OpenDcaV2InstructionArgs},
};
use solana_sdk::{
    get_associated_token_address, get_or_create_associated_token_address, pubkey::Pubkey,
    transaction::VersionedTransaction,
};

pub fn use_recurring_transaction(
    sell_token: Signal<Option<Token>>,
    buy_token: Signal<Option<Token>>,
    sell_token_balance: Resource<GatewayResult<UiTokenAmount>>,
    sell_amount: Signal<u64>,
    sell_amount_count: Signal<u64>,
    sell_frequency: Signal<i64>,
    mut err: Signal<Option<TokenInputError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let gateway = use_gateway();

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Ensure we have sell token
        let Some(sell_token) = sell_token.read().clone() else {
            return Err(GatewayError::Unknown);
        };

        // Ensure we have buy token
        let Some(buy_token) = buy_token.read().clone() else {
            return Err(GatewayError::Unknown);
        };

        // Ensure we have enough sell token balances
        let Some(Ok(sell_token_balance)) = sell_token_balance.read().clone() else {
            err.set(Some(TokenInputError::InsufficientBalance(sell_token)));
            return Err(GatewayError::Unknown);
        };

        // TODO REQUIRE MINIMUM SELL AMOUNT OF 100

        // Check if user's current balance is sufficient to cover the total sell amount
        let sell_token_balance_u64 = sell_token_balance
            .amount
            .parse::<u64>()
            .map_err(|_| GatewayError::Unknown)?;

        if sell_token_balance_u64 < sell_amount.cloned() {
            err.set(Some(TokenInputError::InsufficientBalance(sell_token)));
            return Err(GatewayError::Unknown);
        }

        let ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        let dca_ix = match gateway
            .build_jupiter_dca_instruction(
                authority,
                sell_token,
                buy_token,
                sell_amount,
                sell_amount_per_cycle,
                cycle_frequency,
                start_at,
            )
            .await
        {
            Ok(ix) => ix,
            Err(e) => {
                // err.set(Some(TokenInputError::Unknown(e.to_string())));
                return Err(GatewayError::JupDcaError);
            }
        };

        ixs.extend(dca_ix);

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, 5000));

        // Build initial transaction to estimate priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // Get priority fee estimate
        let gateway = use_gateway();
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
            Ok(fee) => fee,
            Err(_) => {
                log::error!("Failed to fetch priority fee estimate");
                return Err(GatewayError::Unknown);
            }
        };

        // Add priority fee instruction
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(
            dynamic_priority_fee,
        ));

        // Calculate priority fee in lamports
        let adjusted_compute_unit_limit_u64: u64 = COMPUTE_UNIT_LIMIT.into();
        let dynamic_priority_fee_in_lamports =
            (dynamic_priority_fee * adjusted_compute_unit_limit_u64) / 1_000_000;

        // // Set priority fee for UI
        // priority_fee.set(dynamic_priority_fee_in_lamports);

        // Build final tx with priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(VersionedTransaction::default())
    })
}
