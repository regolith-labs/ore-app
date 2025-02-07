use std::str::FromStr;

use dioxus::prelude::*;
use ore_pool_types::MemberChallenge;
use steel::Pubkey;

use crate::{
    gateway::{pool::PoolGateway, GatewayResult}, 
    hooks::{use_gateway, use_member_record, use_miner_status, use_pool_url}
};

use super::{use_miner_is_active, MinerStatus};

pub fn use_next_challenge(last_hash_at: Signal<i64>) -> Resource<GatewayResult<MemberChallenge>> {
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
        use_gateway().poll_new_challenge(
            member_authority,
            pool_url,
            last_hash_at,
        )
        .await
    })
}
