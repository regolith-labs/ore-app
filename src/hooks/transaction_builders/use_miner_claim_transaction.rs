use crate::gateway::Rpc;
use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet, COMPUTE_UNIT_BUFFER},
    solana::{
        spl_associated_token_account::{self, get_associated_token_address},
        spl_token,
    },
};

const ESTIMATED_MINER_CLAIM_COMPUTE_UNITS: u32 = 110000;

pub fn use_miner_claim_transaction(
    member_on_chain: Resource<GatewayResult<ore_pool_api::state::Member>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Destructure member_on_chain
        let data = &*member_on_chain.value().read_unchecked();

        // Get member data from Member
        let member_data = match data {
            Some(Ok(member)) => member, // Extract the Member if successful
            Some(Err(_err)) => return Err(GatewayError::Unknown), // Handle the Err case
            None => return Err(GatewayError::Unknown), // Handle the None case
        };

        // Check if miner has no balance to claim
        if member_data.balance <= 0 {
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // // Adjust compute unit limit based on buffer -> 110,000
        // let adjusted_compute_unit_limit = ESTIMATED_MINER_CLAIM_COMPUTE_UNITS
        //     + (ESTIMATED_MINER_CLAIM_COMPUTE_UNITS as f64 * COMPUTE_UNIT_BUFFER) as u32;

        // log::info!(
        //     "adjusted_compute_unit_limit: {}",
        //     adjusted_compute_unit_limit
        // );

        // // Set compute unit limit
        // ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
        //     adjusted_compute_unit_limit,
        // ));

        let gateway = use_gateway();

        // Get the associated token address for miner
        let ata_address = get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);

        // Check if the associated token account exists
        if gateway.rpc.get_token_account(&ata_address).await.is_err() {
            let create_ata_ix =
                spl_associated_token_account::instruction::create_associated_token_account(
                    &authority,
                    &authority,
                    &ore_api::consts::MINT_ADDRESS,
                    &spl_token::ID,
                );
            // Miner does not have ata, so create associated token account
            // Add create ata instruction to the transactions
            ixs.push(create_ata_ix);
        }

        // Use the ata_address directly since it should now exist
        let beneficiary = ata_address;

        // Add claim transaction
        ixs.push(ore_pool_api::sdk::claim(
            authority,
            beneficiary,
            member_data.pool,
            member_data.balance,
        ));

        // Include ORE app fee
        let treasury_token_address = ore_api::consts::TREASURY_TOKENS_ADDRESS;
        ixs.push(transfer(&authority, &authority, 5000));

        // Build initial transaction to estimate priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // Get priority fee estimate
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
            Ok(fee) => fee,
            Err(_) => return Err(GatewayError::Unknown),
        };

        // Add priority fee instruction
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(
            dynamic_priority_fee,
        ));

        // Build transaction with priority fee
        let tx: VersionedTransaction = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
