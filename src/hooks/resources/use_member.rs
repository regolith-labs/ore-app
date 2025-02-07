use dioxus::prelude::*;
use ore_pool_api::state::Member;
use ore_pool_types::Member as MemberRecord;

use crate::{
    config::Pool, gateway::{pool::PoolGateway, GatewayError, GatewayResult}, hooks::{use_gateway, use_wallet, GetPubkey}
};

pub fn use_member(pool: Resource<Pool>) -> Resource<GatewayResult<Member>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;
        let Some(pool) = pool.cloned() else {
            return Err(GatewayError::AccountNotFound);
        };
        let member_pda = ore_pool_api::state::member_pda(pubkey, pool.address);
        use_gateway().get_member(member_pda.0).await
    })
}

pub fn use_member_record(pool: Resource<Pool>) -> Resource<GatewayResult<MemberRecord>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;
        let Some(pool) = pool.cloned() else {
            return Err(GatewayError::AccountNotFound);
        };
        use_gateway().get_member_record(pubkey, pool.url).await
    })
}