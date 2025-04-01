use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{
        use_gateway, use_member_record_resource_deprecated, use_member_resource_deprecated,
        use_pool, use_pool_deprecated, use_wallet, GetPubkey, APP_FEE, APP_FEE_ACCOUNT,
        COMPUTE_UNIT_LIMIT,
    },
};
use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use super::{use_pool_commit_claim_transaction_submit, CommitClaimStatus};

#[derive(Clone)]
pub enum PoolRegisterStatus {
    CommitClaimFailed,
    Transaction(VersionedTransaction),
}

pub fn use_pool_register_transaction(
    start: Signal<bool>,
) -> Resource<GatewayResult<PoolRegisterStatus>> {
    // wallet
    let wallet = use_wallet();
    // pool and deprecated pool
    let pool = use_pool();
    let pool_deprecated = use_pool_deprecated();
    // deprecated onchain and db member accounts
    let member_deprecated = use_member_resource_deprecated();
    let member_record_deprecated = use_member_record_resource_deprecated();
    // commit and claim from deprecated pool before joining new pool
    let commit_claim_status = use_pool_commit_claim_transaction_submit(
        pool_deprecated,
        member_record_deprecated,
        member_deprecated,
        start,
    );
    use_resource(move || async move {
        // there's a dependency on landing a commit-claim transaction in the deprecated pool
        // before submitting a join instruction into the current pool
        match *commit_claim_status.read() {
            // commit claim landed or was skipped
            CommitClaimStatus::Done | CommitClaimStatus::Skip => {
                if let Some(pool) = pool.cloned() {
                    let pubkey = wallet.pubkey()?;
                    // aggregate instructions
let mut ixs = vec![];
// set compute unit limit
ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
    COMPUTE_UNIT_LIMIT,
));
// build join instruction
let join_ix = ore_pool_api::sdk::join(pubkey, pool.address, pubkey);
ixs.push(join_ix);
// include app fee
let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
ixs.push(transfer(&pubkey, &app_fee_account, APP_FEE));
// build initial transaction to estimate priority fee
let tx = Transaction::new_with_payer(&ixs, Some(&pubkey)).into();
// get priority fee estimate
let gateway = use_gateway();
let dynamic_priority_fee =
    match gateway.get_recent_priority_fee_estimate(&tx).await {
        Ok(fee) => fee,
        Err(_) => {
            log::error!("Failed to fetch priority fee estimate");
            return Err(GatewayError::Unknown);
        }
    };
// set dynamic priority fee (with a default of 100 if it fails)
let priority_fee = dynamic_priority_fee.unwrap_or(100);
let priority_fee_instruction = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
ixs.insert(0, priority_fee_instruction);
// rebuild the transaction with the updated instructions
let tx = Transaction::new_with_payer(&ixs, Some(&pubkey)).into();
Ok(PoolRegisterStatus::Transaction(tx))