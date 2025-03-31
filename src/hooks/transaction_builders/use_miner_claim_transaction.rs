use crate::{
    hooks::{use_pool, Wallet},
    pages::MemberBalance,
};
use dioxus::prelude::*;
use solana_sdk::transaction::Transaction;

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet},
};

use super::build_commit_claim_instructions;

pub fn use_miner_claim_transaction(
    member: Resource<GatewayResult<ore_pool_api::state::Member>>,
    member_db: Resource<GatewayResult<ore_pool_types::Member>>,
    member_claimable_balance: Memo<MemberBalance>,
) -> Resource<GatewayResult<Transaction>> {
    let wallet = use_wallet();
    let pool = use_pool();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Check for pool
        let pool = match pool.cloned() {
            Some(pool) => pool,
            None => return Err(GatewayError::AccountNotFound),
        };

        // Check for member db
        let member_db = match member_db.cloned() {
            Some(Ok(member_db)) => member_db,
            _ => return Err(GatewayError::AccountNotFound),
        };

        // Check for memmber
        let member = match member.cloned() {
            Some(Ok(member)) => member, // Extract the Member if successful
            _ => return Err(GatewayError::Unknown), // Handle the None case
        };

        // Check for claimable balance
        let claimable_balance = match *member_claimable_balance.read() {
            MemberBalance::Balance(balance) => balance,
            _ => return Err(GatewayError::Unknown),
        };

        // Check if miner has no balance to claim
        if claimable_balance <= 0 {
            return Err(GatewayError::Unknown);
        }

        // Build the commit claim instructions
        let gateway = use_gateway();
        let ixs = build_commit_claim_instructions(&gateway.rpc, &pool, &member, &member_db).await?;

        // TODO: add fee account to commit-claim validation logic pool server side
        // // Include ORE app fee
        // let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        // ixs.push(transfer(&authority, &app_fee_account, 5000));

        // TODO: add dynamic fee to commit-claim instruction builder
        // // Build initial transaction to estimate priority fee
        // let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        // // Get priority fee estimate
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

        let tx = Transaction::new_with_payer(&ixs, Some(&authority));
        Ok(tx)
    })
}
