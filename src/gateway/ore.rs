use ore_boost_api::state::{Boost, Stake};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{GatewayError, GatewayResult, Rpc};

pub trait OreGateway {
    async fn get_boost(&self, mint: Pubkey) -> GatewayResult<Boost>;
    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake>;
}

impl<R: Rpc> OreGateway for R {
    async fn get_boost(&self, address: Pubkey) -> GatewayResult<Boost> {
        let data = self
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Boost::try_from_bytes(&data)?)
    }

    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake> {
        let data = self
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Stake::try_from_bytes(&data)?)
    }
}
