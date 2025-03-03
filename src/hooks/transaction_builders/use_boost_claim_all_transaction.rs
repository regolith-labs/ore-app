use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_all_stakes, use_gateway, use_wallet, Wallet, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
    solana::spl_associated_token_account,
};

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

            if ixs.is_empty() {
                return Err(GatewayError::Unknown);
            }

            // Set compute unit limit
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
                COMPUTE_UNIT_LIMIT,
            ));

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

            // Build final tx with priority fee
            let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
            Ok(tx)
        }
    })
}
