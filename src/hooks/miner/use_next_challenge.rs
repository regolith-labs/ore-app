use std::str::FromStr;

use dioxus::prelude::*;
use ore_pool_types::{Member as MemberRecord, MemberChallenge};
use steel::Pubkey;

use crate::{
    hooks::use_gateway,
    gateway::{GatewayResult, pool::PoolGateway},
    config::Pool,
};

pub fn use_next_challenge(
    last_hash_at: Signal<i64>,
    member_record: Resource<GatewayResult<MemberRecord>>,
    pool: Resource<Pool>,
) -> Resource<GatewayResult<MemberChallenge>> {
    use_resource(move || {
        let last_hash_at = *last_hash_at.read();
        async move {
            if let (Some(Ok(member_record)), Some(pool)) = (member_record.cloned(), pool.cloned()) {
                use_gateway().poll_new_challenge(
                    Pubkey::from_str(member_record.authority.as_str()).unwrap(),
                    pool.url,
                    last_hash_at,
                )
                .await
            } else {
                Err(crate::gateway::GatewayError::AccountNotFound)
            }
        }
    })
}