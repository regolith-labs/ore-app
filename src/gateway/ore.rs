use ore_api::{
    consts::CONFIG_ADDRESS,
    state::{proof_pda, Config, Proof},
};
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{GatewayError, GatewayResult, Rpc};

pub trait OreGateway {
    async fn get_config(&self) -> GatewayResult<Config>;
    async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof>;
    async fn get_boost(&self, mint: Pubkey) -> GatewayResult<Boost>;
    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake>;
}

impl<R: Rpc> OreGateway for R {
    async fn get_config(&self) -> GatewayResult<Config> {
        let data = self
            .get_account_data(&CONFIG_ADDRESS)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Config::try_from_bytes(&data)?)
    }

    async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        let data = self
            .get_account_data(&proof_pda(authority).0)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Proof::try_from_bytes(&data)?)
    }

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
