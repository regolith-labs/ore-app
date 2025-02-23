use std::str::FromStr;

use dioxus::prelude::*;
use ore_miner_types::{InputMessage, OutputMessage};
use ore_pool_types::MemberChallenge;
use steel::Pubkey;

use crate::{
    gateway::{pool::PoolGateway, GatewayResult},
    hooks::{
        use_gateway, use_member_record, use_miner, use_miner_is_active, use_miner_status,
        use_pool_url, use_wallet, GetPubkey, MinerStatus, MiningEvent,
    },
};

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
        Some(Pubkey::from_str(member_record.authority.as_str()).unwrap())
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
    let is_active = use_miner_is_active();
    let member_record = use_member_record();
    use_effect(move || {
        if *is_active.read() {
            if let Some(Ok(member_record)) = member_record.cloned() {
                if let Some(Ok(challenge)) = challenge.cloned() {
                    spawn(async move {
                        if let Ok(cutoff_time) = use_gateway()
                            .get_cutoff(challenge.challenge.lash_hash_at, 5)
                            .await
                        {
                            miner_status.set(MinerStatus::Hashing);
                            to_miner.send(ore_miner_types::InputMessage {
                                member: member_record,
                                challenge,
                                cutoff_time,
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
    let mut member_record = use_member_record();
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
                // Submit solution
                spawn(async move {
                    miner_status.set(MinerStatus::SubmittingSolution);
                    if let Err(err) = use_gateway()
                        .post_solution(pubkey, pool_url.clone(), &solution)
                        .await
                    {
                        log::error!("Error posting solution: {:?}", err);
                    }
                    miner_status.set(MinerStatus::Hashing);
                    // TODO:
                    // this is a bug --
                    // restarting the member record (probably to fetch rewards balance?)
                    // will trigger the dispatch challenge resource.
                    // what happens is that the previous challenge is still sitting there
                    // but the dispatcher "thinks" it received a new chalenge
                    // when really only the member record was restarted,
                    // so the miner ends up with a stale (previous) challenge.
                    // this causes a chain reaction where the miner is perpetually stale.
                    //
                    // member_record.restart(); <-- bug
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
            }
            _ => {}
        }
    })
}
