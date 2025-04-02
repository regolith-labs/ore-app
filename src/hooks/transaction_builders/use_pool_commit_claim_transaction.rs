use ore_api::consts::MINT_ADDRESS;
use steel::AccountDeserialize;

use crate::config::Pool;
use crate::gateway::{GatewayError, Rpc};
use crate::solana::spl_associated_token_account;

#[cfg(not(feature = "web"))]
pub async fn build_commit_claim_instructions<R: Rpc>(
    gateway: &R,
    pool: &Pool,
    member: &ore_pool_api::state::Member,
    member_record_balance: u64,
) -> Result<Vec<solana_sdk::instruction::Instruction>, GatewayError> {
    use solana_sdk::compute_budget::ComputeBudgetInstruction;
    let mut instructions = Vec::with_capacity(5);
    // compute budget
    instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
    instructions.push(ComputeBudgetInstruction::set_compute_unit_price(20_000));
    // add core instructions
    let mut core_instructions =
        build_core_commit_claim_instructions(gateway, pool, member, member_record_balance).await?;
    instructions.append(&mut core_instructions);
    // Add jito tip
    instructions.push(tip_ix(&member.authority));
    Ok(instructions)
}

#[cfg(feature = "web")]
pub async fn build_commit_claim_instructions<R: Rpc>(
    gateway: &R,
    pool: &Pool,
    member: &ore_pool_api::state::Member,
    member_record_balance: u64,
) -> Result<Vec<solana_sdk::instruction::Instruction>, GatewayError> {
    // For web, we just use the core instructions directly
    build_core_commit_claim_instructions(gateway, pool, member, member_record_balance).await
}

async fn build_core_commit_claim_instructions<R: Rpc>(
    gateway: &R,
    pool: &Pool,
    member: &ore_pool_api::state::Member,
    member_record_balance: u64,
) -> Result<Vec<solana_sdk::instruction::Instruction>, GatewayError> {
    let mut instructions = Vec::new();
    // commit
    let pool_data = gateway.get_account_data(&pool.address).await?;
    let pool_account = ore_pool_api::state::Pool::try_from_bytes(pool_data.as_slice())?;
    let commit_ix = ore_pool_api::sdk::attribute(
        pool_account.authority,
        member.authority,
        member_record_balance,
    );
    instructions.push(commit_ix);

    // claim
    //
    // 1) check if ata exists or create
    // 2) build claim amount
    // 3) create instruction
    let claim_ata = spl_associated_token_account::get_associated_token_address(
        &member.authority,
        &MINT_ADDRESS,
    );

    // 1) check that beneficiary token account exists o.w. create
    let claim_ata_data = gateway.get_account_data(&claim_ata).await;
    if let Err(_err) = claim_ata_data {
        let create_ata = spl_associated_token_account::instruction::create_associated_token_account(
            &member.authority,
            &member.authority,
            &MINT_ADDRESS,
            &crate::solana::spl_token::ID,
        );
        instructions.push(create_ata);
    };

    // 2) build claim amount
    let diff = member_record_balance as u64 - member.total_balance;
    let claim_amount = member.balance + diff;

    // 3) create claim instruction
    let claim_ix =
        ore_pool_api::sdk::claim(member.authority, claim_ata, pool.address, claim_amount);
    instructions.push(claim_ix);

    Ok(instructions)
}
