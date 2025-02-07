use dioxus::prelude::*;
use solana_sdk::{compute_budget::ComputeBudgetInstruction, transaction::{Transaction, VersionedTransaction}};
use steel::Pubkey;

use crate::{gateway::GatewayResult, hooks::{use_wallet, GetPubkey}};


pub fn use_pool_register_transaction(pool_address: Pubkey) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || {
        async move {
            let pubkey = wallet.pubkey()?;
            let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(20_000);
            let join_ix = ore_pool_api::sdk::join(pubkey, pool_address, pubkey);
            let mut tx = Transaction::new_with_payer(&[compute_budget_ix, join_ix], Some(&pubkey));
            Ok(tx.into())
        }
    })
}