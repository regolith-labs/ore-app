use ore_api::{
    consts::CONFIG_ADDRESS,
    state::{proof_pda, Config, Proof},
};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{retry, Gateway, GatewayError, GatewayResult};

impl Gateway {
    pub async fn get_config(&self) -> GatewayResult<Config> {
        retry(|| async {
            let data = self
                .rpc
                .get_account_data(&CONFIG_ADDRESS)
                .await
                .map_err(GatewayError::from)?;
            Ok(*Config::try_from_bytes(&data).expect("Failed to parse config account"))
        })
        .await
    }

    pub async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        retry(|| async {
            let data = self
                .rpc
                .get_account_data(&proof_pda(authority).0)
                .await
                .map_err(GatewayError::from)?;
            Ok(*Proof::try_from_bytes(&data).expect("Failed to parse proof"))
        })
        .await
    }
}
