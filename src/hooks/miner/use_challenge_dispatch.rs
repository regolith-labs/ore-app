use dioxus::prelude::*;
use ore_miner_types::InputMessage;
use ore_pool_types::MemberChallenge;

use crate::{gateway::{pool::PoolGateway, GatewayResult}, hooks::{use_gateway, use_member_record}};

use super::{use_miner_is_active, use_miner_status, MinerStatus};

pub fn use_challenge_dispatch(
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
                        if let Ok(cutoff_time) = use_gateway().get_cutoff(challenge.challenge.lash_hash_at, 5).await {
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