use dioxus::prelude::*;
use ore_types::Transfer;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::AsyncResult;

use super::{use_gateway, use_pubkey};

pub const ACTIVITY_TABLE_PAGE_LIMIT: usize = 8;

#[derive(Debug)]
pub enum ActivityFilter {
    Global,
    Personal,
}

pub fn use_transfer(sig: String) -> Signal<AsyncResult<Transfer>> {
    let gateway = use_gateway();
    let mut transfer = use_signal(|| AsyncResult::Loading);

    use_future(move || {
        let gateway = gateway.clone();
        let sig = sig.clone();
        async move {
            if let Some(res) = gateway.get_transfer(sig).await {
                transfer.set(AsyncResult::Ok(res));
            }
        }
    });

    transfer
}

pub fn use_user_transfers(
    user_id: Pubkey,
    offset: Signal<u64>,
) -> (Signal<AsyncResult<Vec<Transfer>>>, Signal<bool>) {
    let gateway = use_gateway();
    let mut transfers = use_signal::<AsyncResult<Vec<Transfer>>>(|| AsyncResult::Loading);
    let mut has_more = use_signal(|| false);

    use_future(move || {
        let gateway = gateway.clone();
        let offset = *offset.read();
        async move {
            if let Some(res) = gateway
                .list_transfers(Some(user_id), offset, ACTIVITY_TABLE_PAGE_LIMIT)
                .await
            {
                transfers.set(AsyncResult::Ok(res.data));
                has_more.set(res.has_more);
            };
        }
    });

    (transfers, has_more)
}

pub fn use_transfers(
    filter: Signal<ActivityFilter>,
    offset: Signal<u64>,
) -> (Signal<AsyncResult<Vec<Transfer>>>, Signal<bool>) {
    let gateway = use_gateway();
    let pubkey = use_pubkey();
    let mut transfers = use_signal::<AsyncResult<Vec<Transfer>>>(|| AsyncResult::Loading);
    let mut has_more = use_signal(|| false);

    use_future(move || {
        let gateway = gateway.clone();
        let offset = *offset.read();
        let user = match *filter.read() {
            ActivityFilter::Global => None,
            ActivityFilter::Personal => Some(pubkey),
        };
        async move {
            if let Some(res) = gateway
                .list_transfers(user, offset, ACTIVITY_TABLE_PAGE_LIMIT)
                .await
            {
                transfers.set(AsyncResult::Ok(res.data));
                has_more.set(res.has_more);
            };
        }
    });

    (transfers, has_more)
}
