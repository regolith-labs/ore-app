use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::message::{v0, VersionedMessage};
use solana_sdk::signature::null_signer::NullSigner;
use solana_sdk::transaction::Transaction;
use steel::AccountDeserialize;

use crate::components::{sign_transaction_partial, SecondSigner};
use crate::config::Pool;
use crate::gateway::pool::PoolGateway;
use crate::gateway::{GatewayError, GatewayResult, Rpc};
use crate::hooks::use_gateway;

pub fn use_pool_commit_claim_transaction_submit(
    pool: Resource<Pool>,
    member_record: Resource<GatewayResult<ore_pool_types::Member>>,
    member: Resource<GatewayResult<ore_pool_api::state::Member>>,
    start: Signal<bool>,
) -> Signal<bool> {
    let mut finished = use_signal(|| false);
    use_effect(move || {
        // fired off by register button
        if *start.read() {
            // match on accounts
            if let (Some(pool), Some(Ok(member)), Some(Ok(member_record))) =
                (pool.cloned(), member.cloned(), member_record.cloned())
            {
                spawn(async move {
                    if let Err(err) = async {
                        log::info!("spawned commit claim");
                        let gateway = use_gateway();
                        let mut instructions = Vec::with_capacity(4);
                        // compute budget
                        instructions
                            .push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
                        instructions.push(ComputeBudgetInstruction::set_compute_unit_price(20_000));
                        // commit
                        log::info!("fetching pool data");
                        let pool_data = gateway.rpc.get_account_data(&pool.address).await?;
                        log::info!("pool data: {:?}", pool_data);
                        let pool_account =
                            ore_pool_api::state::Pool::try_from_bytes(pool_data.as_slice())?;
                        log::info!("pool account: {:?}", pool_account);
                        let commit_ix = ore_pool_api::sdk::attribute(
                            pool_account.authority,
                            member.authority,
                            member_record.total_balance as u64,
                        );
                        instructions.push(commit_ix);
                        // claim
                        let claim_ata =
                        crate::solana::spl_associated_token_account::get_associated_token_address(
                            &member.authority,
                            &MINT_ADDRESS,
                        );
                        let diff = member_record.total_balance as u64 - member.total_balance;
                        let claim_amount = member.balance + diff;
                        let claim_ix = ore_pool_api::sdk::claim(
                            member.authority,
                            claim_ata,
                            pool.address,
                            claim_amount,
                        );
                        instructions.push(claim_ix);
                        // build transaction
                        let transaction = Transaction::new_with_payer(
                            instructions.as_slice(),
                            Some(&member.authority),
                        );
                        log::info!("signing partial");
                        // partial sign transaction
                        let (signed, hash) = sign_transaction_partial(transaction).await?;
                        log::info!("posting to server");
                        // post transaction to pool server
                        let update_balance = gateway
                            .commit_claim(member.authority, pool.url, signed, hash)
                            .await?;
                        log::info!("update balance: {:?}", update_balance);
                        finished.set(true);
                        Ok::<_, GatewayError>(())
                    }
                    .await
                    {
                        log::error!("{:?}", err);
                    }
                });
            } else {
                // if the user is not registed in this pool just return immediately as finished
                finished.set(true)
            }
        }
    });
    finished
}
