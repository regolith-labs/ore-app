use dioxus::prelude::*;
use ore_boost_api::state::{boost_pda, Boost};
use steel::Pubkey;

use crate::{config::{BoostMeta, LpType, LISTED_TOKENS}, gateway::{kamino::KaminoGateway, meteora::MeteoraGateway, ore::OreGateway, GatewayError, GatewayResult}};
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
                let amm = use_gateway().get_amm_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS.get(&amm.pool_token_mints[0]).unwrap().ticker.to_string();
                let token_b = LISTED_TOKENS.get(&amm.pool_token_mints[1]).unwrap().ticker.to_string();
                let balance_a = amm.pool_token_amounts[0];
                let balance_b = amm.pool_token_amounts[1];
                let reverse = token_a == "ORE";
                return Ok(BoostDeposits {
                    token_a: if reverse { token_b.clone() } else { token_a.clone() },
                    token_b: if reverse { token_a } else { token_b },
                    balance_a: if reverse { balance_b } else { balance_a },
                    balance_b: if reverse { balance_a } else { balance_b },
                    total_value_usd: amm.pool_tvl,
                });
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
