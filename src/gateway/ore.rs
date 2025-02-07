use ore_boost_api::state::{Boost, Stake};
use ore_pool_api::state::Member;
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{GatewayError, GatewayResult, Rpc};

pub trait OreGateway {
    async fn get_boost(&self, mint: Pubkey) -> GatewayResult<Boost>;
    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake>;
    async fn get_member(&self, pool: Pubkey, authority: Pubkey) -> GatewayResult<Member>;
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

    async fn get_member(&self, authority: Pubkey, pool: Pubkey) -> GatewayResult<Member> {
        let member_pda = ore_pool_api::state::member_pda(authority, pool);
        let data = self
            .get_account_data(&member_pda.0)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Member::try_from_bytes(&data)?)
    }
}
