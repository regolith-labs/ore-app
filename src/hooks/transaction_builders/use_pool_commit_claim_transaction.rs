use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use ore_pool_types::BalanceUpdate;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use steel::AccountDeserialize;

use crate::config::Pool;
use crate::gateway::{GatewayResult, Rpc};
use crate::hooks::use_gateway;

pub fn use_pool_commit_claim_transaction(
    pool: Resource<Pool>,
    member_record: Resource<ore_pool_types::Member>,
    member: Resource<ore_pool_api::state::Member>,
) -> Resource<GatewayResult<BalanceUpdate>> {
    let gateway = use_gateway();
    use_resource(move || async move {
        if let (Some(Ok(pool)), Some(Ok(member_record))) = (pool.cloned(), member_record.cloned()) {
            let mut instructions = Vec::with_capacity(4);
            // compute budget
            instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
            instructions.push(ComputeBudgetInstruction::set_compute_unit_price(20_000));
            // commit
            let pool_data = gateway.rpc.get_account_data(pool.address).await?;
            let pool_account = ore_pool_api::state::Pool::try_from_bytes(pool.as_slice())?;
            let commit_ix = ore_pool_api::sdk::attribute(
                pool_account.authority,
                member_record.authority,
                member_record.total_balance,
            );
            instructions.push(commit_ix);
            // claim
            let claim_ata =
                crate::solana::spl_associated_token_account::get_associated_token_address(
                    &member_record.authority,
                    &MINT_ADDRESS,
                );
            // TODO:
            let claim_ix =
                ore_pool_api::sdk::claim(member_record.authority, claim_ata, pool.address, _);
        }
        Ok(())
    })
}
