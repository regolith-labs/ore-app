use dioxus::prelude::*;
use ore_boost_api::state::{boost_pda, Boost};
use steel::Pubkey;

use crate::{config::{BoostMeta, LpType}, gateway::{kamino::KaminoGateway, ore::OreGateway, GatewayError, GatewayResult}};
use super::use_gateway;

pub fn use_boost(mint: Pubkey) -> Resource<GatewayResult<Boost>> {
    use_resource(move || async move {
        let boost_address = boost_pda(mint).0;
        use_gateway().rpc.get_boost(boost_address).await.map_err(GatewayError::from)
    })
}

pub fn use_boost_deposits(boost_meta: BoostMeta) -> Resource<GatewayResult<BoostDeposits>> {
    let lp_type: LpType = boost_meta.lp_type;
    use_resource(move || async move {
        match lp_type {
            LpType::Kamino => {
                let strategy = use_gateway().get_strategy_metrics(boost_meta.lp_id).await?;
                return Ok(BoostDeposits {
                    token_a: strategy.token_a,
                    token_b: strategy.token_b,
                    balance_a: strategy.vault_balances.token_a.total,
                    balance_b: strategy.vault_balances.token_b.total,
                    total_value_usd: strategy.total_value_locked,
                });
            }
            LpType::Meteora => {
                return Err(GatewayError::Unknown);
            }
        }
    })
}

#[derive(Debug, Clone)]
pub struct BoostDeposits {
    pub token_a: String,
    pub token_b: String,
    pub balance_a: f64,
    pub balance_b: f64,
    pub total_value_usd: f64,
}
