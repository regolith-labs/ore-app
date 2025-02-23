use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_pool, use_wallet, GetPubkey},
};

pub fn use_pool_register_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let pool = use_pool();
    let wallet = use_wallet();
    use_resource(move || async move {
        if let Some(pool) = pool.cloned() {
            let pubkey = wallet.pubkey()?;
            let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(20_000);
            let join_ix = ore_pool_api::sdk::join(pubkey, pool.address, pubkey);
            let tx = Transaction::new_with_payer(&[compute_budget_ix, join_ix], Some(&pubkey));
            Ok(tx.into())
        } else {
            Err(GatewayError::AccountNotFound)
        }
    })
}
