use dioxus::prelude::*;
use ore_types::{response::ListTransfersResponse, Transfer};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::GatewayResult;

use super::{
    use_gateway,
    use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

pub const ACTIVITY_TABLE_PAGE_LIMIT: usize = 8;

#[derive(Debug)]
pub enum ActivityFilter {
    Global,
    Personal,
}

pub fn use_transfer(sig: String) -> Resource<GatewayResult<Transfer>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        let sig = sig.clone();
        async move { gateway.get_transfer(sig).await }
    })
}

pub fn use_user_transfers(
    user_id: Pubkey,
    offset: Signal<u64>,
) -> Resource<GatewayResult<ListTransfersResponse>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            let offset = *offset.read();
            gateway
                .list_transfers(Some(user_id), offset, ACTIVITY_TABLE_PAGE_LIMIT)
                .await
        }
    })
}

pub fn use_transfers(
    filter: Signal<ActivityFilter>,
    offset: Signal<u64>,
) -> Resource<GatewayResult<ListTransfersResponse>> {
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || async move {
        let offset = *offset.read();
        let user = match *filter.read() {
            ActivityFilter::Global => None,
            ActivityFilter::Personal => match *wallet_adapter.read() {
                WalletAdapter::Connected(pubkey) => Some(pubkey),
                WalletAdapter::Disconnected => Some(Pubkey::new_from_array([0; 32])),
            },
        };
        use_gateway()
            .list_transfers(user, offset, ACTIVITY_TABLE_PAGE_LIMIT)
            .await
    })
}
