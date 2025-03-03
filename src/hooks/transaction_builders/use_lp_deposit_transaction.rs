use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
};

pub fn use_lp_deposit_transaction(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get resources
        let Some(Ok(boost)) = *boost.read() else {
            return Err(GatewayError::Unknown);
        };

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Open the stake account, if needed
        if let Some(Ok(_stake)) = stake.read().as_ref() {
            // Do nothing
        } else {
            ixs.push(ore_boost_api::sdk::open(authority, authority, boost.mint));
        }

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, 5000));

        // Deposit LP tokens
        ixs.push(ore_boost_api::sdk::deposit(authority, boost.mint, u64::MAX));

        // Build initial transaction
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
        let _dynamic_priority_fee_in_lamports =
            (dynamic_priority_fee * adjusted_compute_unit_limit_u64) / 1_000_000;

        // Build final transaction
        let tx_with_priority_fee = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx_with_priority_fee)
    })
}
