use ore_api::{
    consts::CONFIG_ADDRESS,
    state::{proof_pda, Config, Proof},
};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{retry, GatewayError, GatewayResult, Rpc};

pub trait OreGateway {
    async fn get_config(&self) -> GatewayResult<Config>;
    async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof>;
}

impl<R: Rpc> OreGateway for R {
    async fn get_config(&self) -> GatewayResult<Config> {
        let data = self
            .get_account_data(&CONFIG_ADDRESS)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Config::try_from_bytes(&data).expect("Failed to parse config account"))
    }

    async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        let data = self
            .get_account_data(&proof_pda(authority).0)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Proof::try_from_bytes(&data).expect("Failed to parse proof"))
    }
}
