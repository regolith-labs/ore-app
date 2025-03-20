use dioxus::prelude::*;
use ore_api::state::Proof;
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{
        use_claimable_yield, use_gateway, use_ore_balance, use_wallet, Wallet, APP_FEE_ACCOUNT,
        COMPUTE_UNIT_LIMIT,
    },
    solana::{
        spl_associated_token_account::{
            get_associated_token_address, instruction::create_associated_token_account,
        },
        spl_token,
    },
};

pub fn use_boost_claim_transaction(
    boost: Signal<GatewayResult<Boost>>,
    boost_proof: Resource<GatewayResult<Proof>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let claimable_yield = use_claimable_yield(boost, boost_proof, stake);
    let ore_balance = use_ore_balance();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get resources
        let Some(Ok(_stake)) = *stake.read() else {
            return Err(GatewayError::Unknown);
        };
        let Ok(boost) = *boost.read() else {
            return Err(GatewayError::Unknown);
        };

        // Check if stake has rewards to claim
        if claimable_yield.cloned() == 0 {
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Derive beneficiary
        let beneficiary = get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);

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

        // Claim rewards
        ixs.push(ore_boost_api::sdk::claim(
            authority,
            beneficiary,
            boost.mint,
            claimable_yield.cloned(),
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
    })
}
