use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use solana_sdk::transaction::Transaction;
use steel::AccountDeserialize;

use crate::components::{sign_transaction_partial, TransactionStatus};
use crate::config::Pool;
use crate::gateway::pool::PoolGateway;
use crate::gateway::{GatewayError, GatewayResult, Rpc};
use crate::hooks::{use_gateway, use_transaction_status};
use crate::solana::spl_associated_token_account;

pub fn use_pool_commit_claim_transaction_submit(
    pool: Resource<Pool>,
    member_record_balance: Resource<GatewayResult<u64>>,
    member: Resource<GatewayResult<ore_pool_api::state::Member>>,
    start: Signal<bool>,
) -> Signal<CommitClaimStatus> {
    let mut status = use_signal(|| CommitClaimStatus::Init);
    let mut transaction_status = use_transaction_status();
    use_effect(move || {
        // fired off by register button
        if *start.read() {
            // match on accounts
            if let (Some(pool), Some(Ok(member)), Some(Ok(member_record_balance))) = (
                pool.cloned(),
                member.cloned(),
                member_record_balance.cloned(),
            ) {
                spawn(async move {
                    if let Err(err) = async {
                        transaction_status.set(Some(TransactionStatus::Waiting));
                        let gateway = use_gateway();
                        let instructions = build_commit_claim_instructions(
                            &gateway.rpc,
                            &pool,
                            &member,
                            member_record_balance,
                        )
                        .await?;
                        // build transaction
                        let transaction = Transaction::new_with_payer(
                            instructions.as_slice(),
                            Some(&member.authority),
                        );
                        // partial sign transaction
                        let (signed, hash) = sign_transaction_partial(transaction).await?;
                        // post transaction to pool server
                        let update_balance = gateway
                            .commit_claim(member.authority, pool.url, signed, hash)
                            .await?;
                        transaction_status
                            .set(Some(TransactionStatus::Done(update_balance.signature)));
                        status.set(CommitClaimStatus::Done);
                        Ok::<_, GatewayError>(())
                    }
                    .await
                    {
                        log::error!("{:?}", err);
                        transaction_status.set(Some(TransactionStatus::Error));
                        status.set(CommitClaimStatus::CaughtError);
                    }
                });
            } else {
                // if the user is not registed in this pool just return immediately as finished
                status.set(CommitClaimStatus::Skip);
            }
        }
    });
    status
}

#[cfg(not(feature = "web"))]
pub async fn build_commit_claim_instructions<R: Rpc>(
    gateway: &R,
    pool: &Pool,
    member: &ore_pool_api::state::Member,
    member_record_balance: u64,
) -> Result<Vec<solana_sdk::instruction::Instruction>, GatewayError> {
    use solana_sdk::compute_budget::ComputeBudgetInstruction;
    let mut instructions = Vec::with_capacity(4);
    // compute budget
    instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
    instructions.push(ComputeBudgetInstruction::set_compute_unit_price(20_000));
    // add core instructions
    let mut core_instructions =
        build_core_commit_claim_instructions(gateway, pool, member, member_record_balance).await?;
    instructions.append(&mut core_instructions);
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

#[derive(Copy, Clone)]
pub enum CommitClaimStatus {
    Init,
    Done,
    Skip,
    CaughtError,
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
    log::info!(
        "builder: member db total balance: {:?}",
        member_record_balance
    );
    log::info!("builder: member total balance: {}", member.total_balance);
    log::info!("builder: member balance: {}", member.balance);
    log::info!("builder: diff: {}", diff);
    log::info!("builder: claim amount: {}", claim_amount);
    // 3) create claim instruction
    let claim_ix =
        ore_pool_api::sdk::claim(member.authority, claim_ata, pool.address, claim_amount);
    instructions.push(claim_ix);

    Ok(instructions)
}
