use base64::{prelude::BASE64_STANDARD, Engine};
use dioxus::prelude::*;
use ore_pool_api::state::{member_pda, Member};
use ore_pool_types::Member as MemberRecord;
use steel::AccountDeserialize;

use crate::{
    gateway::{pool::PoolGateway, AccountNotificationParams, GatewayError, GatewayResult},
    hooks::{use_gateway, use_pool, use_pool_deprecated, use_wallet, GetPubkey, Wallet},
};

use super::use_wss_subscription;

pub(crate) fn use_members_provider() {
    let sub = use_member_wss();
    use_context_provider::<Signal<GatewayResult<Member>>>(|| sub);
    let r = use_member_record_resource();
    use_context_provider::<Resource<GatewayResult<MemberRecord>>>(|| r);
    let r = use_member_record_balance_resource();
    use_context_provider::<Resource<GatewayResult<u64>>>(|| r);
}

pub fn use_member() -> Signal<GatewayResult<Member>> {
    use_context()
}

fn use_member_wss() -> Signal<GatewayResult<Member>> {
    let wallet = use_wallet();
    let pool = use_pool();
    // init signal
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    use_effect(move || {
        if let (Wallet::Connected(pubkey), Some(pool)) = (wallet.cloned(), pool.cloned()) {
            let address = member_pda(pubkey, pool.address).0;
            spawn(async move {
                let member = use_gateway().get_member(address).await;
                data.set(member);
            });
        } else {
            log::error!("missing member sub");
        }
    });
    // notif callback
    fn update_callback(notif: &AccountNotificationParams) -> GatewayResult<Member> {
        let data = &notif.result.value.data;
        log::info!("decoding notif data: {:?}", data);
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;
        let member = Member::try_from_bytes(data.as_slice())?;
        Ok(*member)
    }
    // subscribe
    let subscriber = use_wss_subscription(data, update_callback);
    use_effect(move || {
        if let (Wallet::Connected(pubkey), Some(pool)) = (wallet.cloned(), pool.cloned()) {
            let address = member_pda(pubkey, pool.address).0;
            subscriber.send(address);
        }
    });
    data
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
