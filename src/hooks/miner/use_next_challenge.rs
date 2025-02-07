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
    let member_authority = use_memo(move || {
        let Some(Ok(member_record)) = member_record.cloned() else {
            return None;
        };
        Some(Pubkey::from_str(member_record.authority.as_str()).unwrap())
    });

    let pool_url = use_memo(move || {
        let Some(pool) = pool.cloned() else {
            return None;
        };
        Some(pool.url)
    });

    use_resource(move || async move {
        let Some(member_authority) = member_authority.cloned() else {
            return Err(crate::gateway::GatewayError::AccountNotFound);
        };
        let Some(pool_url) = pool_url.cloned() else {
            return Err(crate::gateway::GatewayError::AccountNotFound);
        };
        let last_hash_at = *last_hash_at.read();
        let x = use_gateway().poll_new_challenge(
            member_authority,
            pool_url,
            last_hash_at,
        )
        .await;
        log::info!("Next challenge: {:?}", x);
        x
    })
}
