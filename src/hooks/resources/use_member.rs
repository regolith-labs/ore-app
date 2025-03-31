use dioxus::prelude::*;
use ore_pool_api::state::Member;
use ore_pool_types::Member as MemberRecord;

use crate::{
    gateway::{pool::PoolGateway, GatewayError, GatewayResult},
    hooks::{use_gateway, use_pool, use_pool_deprecated, use_wallet, GetPubkey},
};

pub(crate) fn use_members_provider() {
    let r = use_member_resource();
    use_context_provider::<Resource<GatewayResult<Member>>>(|| r);
    let r = use_member_record_resource();
    use_context_provider::<Resource<GatewayResult<MemberRecord>>>(|| r);
    let r = use_member_record_balance_resource();
    use_context_provider::<Resource<GatewayResult<u64>>>(|| r);
}

fn use_member_resource() -> Resource<GatewayResult<Member>> {
    let pool = use_pool();
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

pub fn use_member_resource_deprecated() -> Resource<GatewayResult<Member>> {
    let pool = use_pool_deprecated();
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

pub fn use_member() -> Resource<GatewayResult<Member>> {
    use_context()
}

pub fn use_member_record_resource_deprecated() -> Resource<GatewayResult<MemberRecord>> {
    let pool = use_pool_deprecated();
    let wallet = use_wallet();
    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;
        let Some(pool) = pool.cloned() else {
            return Err(GatewayError::AccountNotFound);
        };
        use_gateway().get_member_record(pubkey, pool.url).await
    })
}

fn use_member_record_resource() -> Resource<GatewayResult<MemberRecord>> {
    let pool = use_pool();
    let wallet = use_wallet();
    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;
        let Some(pool) = pool.cloned() else {
            return Err(GatewayError::AccountNotFound);
        };
        use_gateway().get_member_record(pubkey, pool.url).await
    })
}

pub fn use_member_record() -> Resource<GatewayResult<MemberRecord>> {
    use_context()
}

fn use_member_record_balance_resource() -> Resource<GatewayResult<u64>> {
    let pool = use_pool();
    let wallet = use_wallet();
    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;
        let Some(pool) = pool.cloned() else {
            return Err(GatewayError::AccountNotFound);
        };
        let member_record = use_gateway().get_member_record(pubkey, pool.url).await?;
        Ok(member_record.total_balance as u64)
    })
}

pub fn use_member_record_balance() -> Resource<GatewayResult<u64>> {
    use_context()
}
