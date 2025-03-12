use std::str::FromStr;

use dioxus::prelude::*;
use ore_miner_types::{InputMessage, OutputMessage};
use ore_pool_types::MemberChallenge;
use steel::Pubkey;

use crate::{
    gateway::{pool::PoolGateway, GatewayResult},
    hooks::{
        use_gateway, use_member_record, use_member_record_balance, use_miner, use_miner_is_active,
        use_miner_status, use_pool_url, use_wallet, GetPubkey, MinerStatus, MiningEvent,
    },
};

use super::use_miner_cores;

pub fn use_mining_loop() {
    // Miner pub/sub channels
    let (from_miner, mut to_miner) = use_miner();
    let last_hash_at = use_signal(|| 0);

    // Miner loop (fetch challenge, dispatch to miner, post solutions)
    let challenge = use_next_challenge(last_hash_at);
    use_challenge_dispatch(challenge, to_miner);
    use_solution_contribute(last_hash_at, from_miner);

    // Restart miner coroutine
    let is_active = use_miner_is_active();
    use_effect(move || {
        if *is_active.read() {
            to_miner.restart();
        }
    });
}

fn use_next_challenge(last_hash_at: Signal<i64>) -> Resource<GatewayResult<MemberChallenge>> {
    let pool_url = use_pool_url();
    let member_record = use_member_record();

    let member_authority = use_memo(move || {
        let Some(Ok(member_record)) = member_record.cloned() else {
            return None;
        };
        Pubkey::from_str(member_record.authority.as_str()).ok()
    });

    let is_active = use_miner_is_active();
    let mut miner_status = use_miner_status();

    use_resource(move || async move {
        let Some(member_authority) = member_authority.cloned() else {
            return Err(crate::gateway::GatewayError::AccountNotFound);
        };
        let Some(pool_url) = pool_url.cloned() else {
            return Err(crate::gateway::GatewayError::AccountNotFound);
        };
        if !*is_active.read() {
            return Err(crate::gateway::GatewayError::Unknown);
        }

        let last_hash_at = *last_hash_at.read();

        miner_status.set(MinerStatus::FetchingChallenge);
        use_gateway()
            .poll_new_challenge(member_authority, pool_url, last_hash_at)
            .await
    })
}

fn use_challenge_dispatch(
    challenge: Resource<GatewayResult<MemberChallenge>>,
    to_miner: Coroutine<InputMessage>,
) -> Effect {
    let mut miner_status = use_miner_status();
    let miner_cores = use_miner_cores();
    let is_active = use_miner_is_active();
    let member_record = use_member_record();
    use_effect(move || {
        if *is_active.read() {
            if let Some(Ok(member_record)) = member_record.cloned() {
                if let Some(Ok(challenge)) = challenge.cloned() {
                    spawn(async move {
                        log::info!("challenge: {:?}", challenge);
                        if let Ok(cutoff_time) = use_gateway()
                            .get_cutoff(
                                challenge.challenge.lash_hash_at,
                                challenge.unix_timestamp,
                                5,
                            )
                            .await
                        {
                            miner_status.set(MinerStatus::Hashing);
                            to_miner.send(ore_miner_types::InputMessage {
                                member: member_record,
                                challenge,
                                cutoff_time,
                                cores: miner_cores.peek().clone(),
                            });
                        }
                    });
                }
            }
        }
    })
}

fn use_solution_contribute(
    mut last_hash_at: Signal<i64>,
    from_miner: Signal<OutputMessage>,
) -> Effect {
    let wallet = use_wallet();
    let pool_url = use_pool_url();
    let mut member_record_balance = use_member_record_balance();
    let mut miner_status = use_miner_status();
    let is_active = use_miner_is_active();
    use_effect(move || {
        // Check status
        let Ok(pubkey) = wallet.pubkey() else {
            return;
        };
        let Some(pool_url) = pool_url.cloned() else {
            return;
        };
        if !*is_active.read() {
            return;
        }
        // Process messsage from miner
        match *from_miner.read() {
            OutputMessage::Solution(solution) => {
                log::info!("received solution");
                // Submit solution
                spawn(async move {
                    log::info!("submitting solution");
                    // Submitting
                    miner_status.set(MinerStatus::SubmittingSolution);
                    if let Err(err) = use_gateway()
                        .post_solution(pubkey, pool_url.clone(), &solution)
                        .await
                    {
                        log::error!("Error posting solution: {:?}", err);
                    }
                    // Set miner status back to hashing
                    // continuous submissions ...
                    miner_status.set(MinerStatus::Hashing);
                    // Restart member balance for pending rewards
                    member_record_balance.restart();
                    // Get latest mine event
                    match use_gateway()
                        .get_latest_event(pubkey, pool_url.clone())
                        .await
                    {
                        Ok(latest_event) => {
                            MiningEvent::add_to_signal(latest_event);
                        }
                        Err(err) => {
                            log::error!("Error getting latest event: {:?}", err);
                        }
                    }
                });
            }
            OutputMessage::Expired(lha) => {
                // Update last hash at
                let peek = *last_hash_at.peek();
                if lha > peek {
                    last_hash_at.set(lha);
                }
                // Poll for latest mine event
                spawn(async move {
                    // Restart member balance for pending rewards
                    member_record_balance.restart();
                    // Get latest mine event
                    match use_gateway()
                        .get_latest_event(pubkey, pool_url.clone())
                        .await
                    {
                        Ok(latest_event) => {
                            MiningEvent::add_to_signal(latest_event);
                        }
                        Err(err) => {
                            log::error!("Error getting latest event: {:?}", err);
                        }
                    }
                });
            }
            _ => {}
        }
    })
}
