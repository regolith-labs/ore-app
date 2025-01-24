use ore_api::{
    consts::CONFIG_ADDRESS,
    state::{proof_pda, Config, Proof},
};
use ore_boost_api::state::{directory_pda, Boost, Directory};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{GatewayError, GatewayResult, Rpc};

pub trait OreGateway {
    async fn get_config(&self) -> GatewayResult<Config>;
    async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof>;
    async fn get_boosts(&self) -> GatewayResult<Vec<Boost>>;
    async fn get_directory(&self) -> GatewayResult<Directory>;
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

    async fn get_boosts(&self) -> GatewayResult<Vec<Boost>> {
        let directory = self.get_directory().await?;
        let mut boosts = vec![];
        for address in directory.boosts {
            if address != Pubkey::default() {
                let data = self
                    .get_account_data(&address)
                    .await
                    .map_err(GatewayError::from)?;
                boosts.push(*Boost::try_from_bytes(&data)?);
            } else {
                break;
            }
        }
        Ok(boosts)
    }

    async fn get_directory(&self) -> GatewayResult<Directory> {
        let data = self
            .get_account_data(&directory_pda().0)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Directory::try_from_bytes(&data)?)
    }
}
