use dioxus::prelude::*;
use ore_miner_types::InputMessage;
use ore_pool_types::{MemberChallenge, Member as MemberRecord};

use crate::{gateway::{GatewayResult, pool::PoolGateway}, hooks::use_gateway};

use super::use_miner_is_active;

pub fn use_challenge_dispatch(
    challenge: Resource<GatewayResult<MemberChallenge>>,
    member_record: Resource<GatewayResult<MemberRecord>>,
    to_miner: Coroutine<InputMessage>,
) -> Effect {
    let is_active = use_miner_is_active();
    use_effect(move || {
        if is_active.read().0 {
            if let Some(Ok(member_record)) = member_record.cloned() {
                if let Some(Ok(challenge)) = challenge.cloned() {
                    spawn(async move {
                        if let Ok(cutoff_time) = use_gateway().get_cutoff(challenge.challenge.lash_hash_at, 5).await {
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