use dioxus::prelude::*;
use ore_boost_api::state::{boost_pda, stake_pda, Boost, Stake};
use steel::Pubkey;

use crate::gateway::{ore::OreGateway, GatewayError, GatewayResult};
use super::{use_gateway, use_wallet, Wallet};

pub fn use_boost(mint: Pubkey) -> Resource<GatewayResult<Boost>> {
    use_resource(move || async move {
        let boost_address = boost_pda(mint).0;
        use_gateway().rpc.get_boost(boost_address).await.map_err(GatewayError::from)
    })
}

pub fn use_stake(mint: Pubkey) -> Resource<GatewayResult<Stake>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        match *wallet.read() {
            Wallet::Disconnected => Err(GatewayError::WalletDisconnected),
            Wallet::Connected(address) => {
                let boost_address = boost_pda(mint).0;
                let stake_address = stake_pda(address, boost_address).0;
                use_gateway().rpc.get_stake(stake_address).await.map_err(GatewayError::from)
            }
        }
    })
}
