use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{
        calculate_claimable_yield, use_all_boosts, use_all_stakes, use_boost_config_wss,
        use_boost_proof_wss, use_ore_balance, use_wallet, Wallet, APP_FEE_ACCOUNT,
        COMPUTE_UNIT_LIMIT,
    },
    solana::{
        spl_associated_token_account::{
            get_associated_token_address, instruction::create_associated_token_account,
        },
        spl_token,
    },
};

#[cfg(not(feature = "web"))]
use super::tip_ix;

pub fn use_boost_claim_all_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let boosts = use_all_boosts();
    let boost_proof = use_boost_proof_wss();
    let boost_config = use_boost_config_wss();
    let stakes = use_all_stakes();
    let ore_balance = use_ore_balance();
    use_resource(move || {
        let boosts = boosts.clone();
        let boost_proof = boost_proof.clone();
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
                if let Ok(stake) = stake.cloned() {
                    let boost = boosts.get(&stake.boost).unwrap();
                    if let Ok(boost) = boost.cloned() {
                        if let Ok(boost_proof) = boost_proof.cloned() {
                            if let Ok(boost_config) = boost_config.cloned() {
                                let claimable_yield = calculate_claimable_yield(
                                    boost,
                                    boost_proof,
                                    stake,
                                    boost_config,
                                );
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

            #[cfg(not(feature = "web"))]
            // Add jito tip
            ixs.push(tip_ix(&authority));

            // Build tx
            let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
            Ok(tx)
        }
    })
}
