use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_all_stakes, use_gateway, use_wallet, Wallet, COMPUTE_UNIT_BUFFER},
    solana::spl_associated_token_account,
};

const ESTIMATED_BOOST_CLAIM_ALL_COMPUTE_UNITS: u32 = 25906;

pub fn use_boost_claim_all_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let stake_accounts = use_all_stakes();
    use_resource(move || {
        let stake_accounts = stake_accounts.clone();
        async move {
            // Check if wallet is connected
            let Wallet::Connected(authority) = *wallet.read() else {
                return Err(GatewayError::WalletDisconnected);
            };

            // Derive beneficiary
            let beneficiary = spl_associated_token_account::get_associated_token_address(
                &authority,
                &ore_api::consts::MINT_ADDRESS,
            );

            // Get resources
            let mut ixs = vec![];
            for (pubkey, stake) in stake_accounts.iter() {
                if let Some(Ok(stake)) = stake.cloned() {
                    if stake.rewards > 0 {
                        ixs.push(ore_boost_api::sdk::claim(
                            authority,
                            beneficiary,
                            *pubkey,
                            stake.rewards,
                        ));
                    }
                }
            }

            // Q: Should we be including this in all builders?
            if ixs.is_empty() {
                return Err(GatewayError::Unknown);
            }

            // Adjust compute unit limit based on buffer -> 110,000
            let adjusted_compute_unit_limit = ESTIMATED_BOOST_CLAIM_ALL_COMPUTE_UNITS
                + (ESTIMATED_BOOST_CLAIM_ALL_COMPUTE_UNITS as f64 * COMPUTE_UNIT_BUFFER) as u32;

            log::info!(
                "adjusted_compute_unit_limit: {}",
                adjusted_compute_unit_limit
            );

            // Set compute unit limit
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
                adjusted_compute_unit_limit,
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
        }
    })
}
