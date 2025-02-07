use dioxus::prelude::*;
use ore_pool_api::state::Member;
use solana_sdk::pubkey::Pubkey;

use crate::{
    gateway::{GatewayResult, ore::OreGateway},
    hooks::{use_gateway, use_wallet, GetPubkey}
};

pub fn use_member(pool_address: Pubkey) -> Resource<GatewayResult<Member>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;
        use_gateway().rpc.get_member(pubkey, pool_address).await
    })
}
