use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet, COMPUTE_UNIT_BUFFER},
    solana::spl_associated_token_account,
};

const ESTIMATED_BOOST_CLAIM_COMPUTE_UNITS: u32 = 13778;

pub fn use_boost_claim_transaction(
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
        let Some(Ok(stake)) = *stake.read() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(boost)) = *boost.read() else {
            return Err(GatewayError::Unknown);
        };

        // Check if stake has rewards to claim
        if stake.rewards == 0 {
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Adjust compute unit limit based on buffer -> 110,000
        let adjusted_compute_unit_limit = ESTIMATED_BOOST_CLAIM_COMPUTE_UNITS
            + (ESTIMATED_BOOST_CLAIM_COMPUTE_UNITS as f64 * COMPUTE_UNIT_BUFFER) as u32;

        log::info!(
            "adjusted_compute_unit_limit: {}",
            adjusted_compute_unit_limit
        );

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            adjusted_compute_unit_limit,
        ));

        // Derive beneficiary
        let beneficiary = spl_associated_token_account::get_associated_token_address(
            &authority,
            &ore_api::consts::MINT_ADDRESS,
        );

        // Claim rewards
        ixs.push(ore_boost_api::sdk::claim(
            authority,
            beneficiary,
            boost.mint,
            stake.rewards,
        ));

        // Include ORE app fee
        let treasury_token_address = ore_api::consts::TREASURY_TOKENS_ADDRESS;
        ixs.push(transfer(&authority, &authority, 5000));

        // Build initial transaction to estimate priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // Get priority fee estimate
        let gateway = use_gateway();
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
            Ok(fee) => fee,
            Err(_) => return Err(GatewayError::Unknown),
        };

        // Add priority fee instruction
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(
            dynamic_priority_fee,
        ));

        // Build transaction with priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
