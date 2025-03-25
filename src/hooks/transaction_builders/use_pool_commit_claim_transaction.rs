use ore_api::consts::MINT_ADDRESS;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::message::{v0, VersionedMessage};
use solana_sdk::signature::null_signer::NullSigner;
use solana_sdk::transaction::VersionedTransaction;
use steel::AccountDeserialize;

use crate::components::SecondSigner;
use crate::config::Pool;
use crate::gateway::{GatewayResult, Rpc};
use crate::hooks::use_gateway;

pub async fn use_pool_commit_claim_transaction(
    pool: Pool,
    member_record: ore_pool_types::Member,
    member: ore_pool_api::state::Member,
) -> GatewayResult<(VersionedTransaction, SecondSigner)> {
    let gateway = use_gateway();
    let mut instructions = Vec::with_capacity(4);
    // compute budget
    instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
    instructions.push(ComputeBudgetInstruction::set_compute_unit_price(20_000));
    // commit
    let pool_data = gateway.rpc.get_account_data(&pool.address).await?;
    let pool_account = ore_pool_api::state::Pool::try_from_bytes(pool_data.as_slice())?;
    let commit_ix = ore_pool_api::sdk::attribute(
        pool_account.authority,
        member.authority,
        member_record.total_balance as u64,
    );
    instructions.push(commit_ix);
    // claim
    let claim_ata = crate::solana::spl_associated_token_account::get_associated_token_address(
        &member.authority,
        &MINT_ADDRESS,
    );
    let diff = member_record.total_balance as u64 - member.total_balance;
    let claim_amount = member.balance + diff;
    let claim_ix =
        ore_pool_api::sdk::claim(member.authority, claim_ata, pool.address, claim_amount);
    instructions.push(claim_ix);
    // build transaction
    let hash = gateway.rpc.get_latest_blockhash().await?;
    let message = v0::Message::try_compile(&member.authority, instructions.as_slice(), &[], hash)?;
    let member_authority = NullSigner::new(&member.authority);
    let pool_authority = NullSigner::new(&pool_account.authority);
    let transaction = VersionedTransaction::try_new(
        VersionedMessage::V0(message),
        &[&member_authority, &pool_authority],
    )?;
    let second_signer = SecondSigner {
        signer: pool_authority,
        payer: false,
    };
    Ok((transaction, second_signer))
}
