use dioxus::prelude::*;
use dioxus_std::utils::rw::use_rw;
use ore_types::Transfer;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{components::ActivityFilter, gateway::AsyncResult};

use super::{use_gateway, use_pubkey, use_transfers_websocket};

pub const ACTIVITY_TABLE_PAGE_LIMIT: usize = 8;

pub fn use_transfer(cx: &ScopeState, sig: String) -> AsyncResult<Transfer> {
    let gateway = use_gateway(cx);
    let transfer = use_state(cx, || AsyncResult::Loading);

    let _ = use_future(cx, (), |_| {
        let gateway = gateway.clone();
        let transfer = transfer.clone();
        async move {
            if let Some(res) = gateway.get_transfer(sig).await {
                transfer.set(AsyncResult::Ok(res));
            }
        }
    });

    transfer.get().clone()
}

pub fn use_user_transfers(
    cx: &ScopeState,
    user_id: Pubkey,
    offset: &UseState<u64>,
) -> (AsyncResult<Vec<Transfer>>, bool) {
    let gateway = use_gateway(cx);
    let transfers = use_rw::<AsyncResult<Vec<Transfer>>>(cx, || AsyncResult::Loading);
    let has_more = use_state(cx, || false);

    let _ = use_future(cx, &offset.clone(), |_| {
        let gateway = gateway.clone();
        let transfers = transfers.clone();
        let has_more = has_more.clone();
        let offset = *offset.current();
        async move {
            if let Some(res) = gateway
                .list_transfers(Some(user_id), offset, ACTIVITY_TABLE_PAGE_LIMIT)
                .await
            {
                transfers.write(AsyncResult::Ok(res.data)).unwrap();
                has_more.set(res.has_more);
            };
        }
    });

    (transfers.read().unwrap().clone(), *has_more.get())
}

pub fn use_transfers(
    cx: &ScopeState,
    filter: &UseState<ActivityFilter>,
    offset: &UseState<u64>,
) -> (AsyncResult<Vec<Transfer>>, bool) {
    let gateway = use_gateway(cx);
    let pubkey = use_pubkey(cx);
    let transfers = use_rw::<AsyncResult<Vec<Transfer>>>(cx, || AsyncResult::Loading);
    let has_more = use_state(cx, || false);

    use_transfers_websocket(
        cx,
        filter,
        transfers,
        offset,
        has_more,
        ACTIVITY_TABLE_PAGE_LIMIT,
    );

    let _ = use_future(cx, (&filter.clone(), &offset.clone()), |_| {
        let gateway = gateway.clone();
        let transfers = transfers.clone();
        let has_more = has_more.clone();
        let offset = *offset.current();
        let user = match filter.get() {
            ActivityFilter::Global => None,
            ActivityFilter::Personal => Some(pubkey),
        };
        async move {
            if let Some(res) = gateway
                .list_transfers(user, offset, ACTIVITY_TABLE_PAGE_LIMIT)
                .await
            {
                transfers.write(AsyncResult::Ok(res.data)).unwrap();
                has_more.set(res.has_more);
            };
        }
    });

    (transfers.read().unwrap().clone(), *has_more.get())
}
