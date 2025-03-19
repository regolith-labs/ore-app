use dioxus::prelude::*;
use ore_api::state::proof_pda;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{
        calculate_claimable_yield, use_all_boost_proofs, use_all_boosts, use_all_stakes,
        use_gateway, use_ore_balance, use_wallet, Wallet, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT,
    },
    solana::{
        spl_associated_token_account::{
            get_associated_token_address, instruction::create_associated_token_account,
        },
        spl_token,
    },
};

pub fn use_boost_claim_all_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let boosts = use_all_boosts();
    let boost_proofs = use_all_boost_proofs();
    let stakes = use_all_stakes();
    let ore_balance = use_ore_balance();
    use_resource(move || {
        let boosts = boosts.clone();
        let boost_proofs = boost_proofs.clone();
        let stakes = stakes.clone();
        async move {
            // Check if wallet is connected
            let Wallet::Connected(authority) = *wallet.read() else {
                return Err(GatewayError::WalletDisconnected);
            };

            // Derive beneficiary
            let beneficiary =
                get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);

            // Create instruction list
            let mut ixs = vec![];

            // Create associated token account if necessary
            if let Some(Ok(_balance)) = ore_balance.cloned() {
                // No op
            } else {
                ixs.push(create_associated_token_account(
                    &authority,
                    &authority,
                    &ore_api::consts::MINT_ADDRESS,
                    &spl_token::ID,
                ));
            }

            // Get resources
            for (_pubkey, stake) in stakes.iter() {
                if let Some(Ok(stake)) = stake.cloned() {
                    let boost = boosts.get(&stake.boost).unwrap();
                    if let Some(Ok(boost)) = boost.cloned() {
                        let proof_address = proof_pda(stake.boost).0;
                        let boost_proof = boost_proofs.get(&proof_address).unwrap();
                        if let Some(Ok(boost_proof)) = boost_proof.cloned() {
                            let claimable_yield =
                                calculate_claimable_yield(boost, boost_proof, stake);
                            if claimable_yield > 0 {
                                ixs.push(ore_boost_api::sdk::claim(
                                    authority,
                                    beneficiary,
                                    boost.mint,
                                    claimable_yield,
                                ));
                            }
                        }
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
            ixs.insert(
                1,
                ComputeBudgetInstruction::set_compute_unit_price(dynamic_priority_fee),
            );

            // Build final tx with priority fee
            let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
            Ok(tx)
        }
    })
}
