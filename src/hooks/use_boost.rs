use dioxus::prelude::*;
use ore_boost_api::state::{boost_pda, Boost};
use steel::Pubkey;

use crate::{config::{BoostMeta, LpType, Token, LISTED_TOKENS, LISTED_TOKENS_BY_TICKER}, gateway::{kamino::KaminoGateway, meteora::MeteoraGateway, ore::OreGateway, GatewayError, GatewayResult, Rpc}};
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
        // Get lp mint supply
        let lp_mint_supply = use_gateway().rpc.get_token_supply(&boost_meta.lp_mint).await?;

        // Get strategy metrics
        match lp_type {
            LpType::Kamino => {
                let strategy = use_gateway().get_kamino_strategy_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS_BY_TICKER.get(&strategy.token_a).unwrap();
                let token_b = LISTED_TOKENS_BY_TICKER.get(&strategy.token_b).unwrap();
                return Ok(BoostDeposits {
                    token_a: token_a.clone(),
                    token_b: token_b.clone(),
                    balance_a_f64: strategy.vault_balances.token_a.total,
                    balance_b_f64: strategy.vault_balances.token_b.total,
                    total_value_usd: strategy.total_value_locked,
                    shares: lp_mint_supply.amount.parse::<u64>().unwrap_or(0),
                });
            }
            LpType::Meteora => {
                let amm = use_gateway().get_meteora_amm_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS.get(&amm.pool_token_mints[0]).unwrap();
                let token_b = LISTED_TOKENS.get(&amm.pool_token_mints[1]).unwrap();
                let balance_a = amm.pool_token_amounts[0];
                let balance_b = amm.pool_token_amounts[1];
                let reverse = token_a.ticker == "ORE";
                return Ok(BoostDeposits {
                    token_a: if reverse { token_b.clone() } else { token_a.clone() },
                    token_b: if reverse { token_a.clone() } else { token_b.clone() },
                    balance_a_f64: if reverse { balance_b } else { balance_a },
                    balance_b_f64: if reverse { balance_a } else { balance_b },
                    total_value_usd: amm.pool_tvl,
                    shares: lp_mint_supply.amount.parse::<u64>().unwrap_or(0),
                });
            }
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoostDeposits {
    pub token_a: Token,
    pub token_b: Token,
    pub balance_a_f64: f64,
    pub balance_b_f64: f64,
    pub total_value_usd: f64,
    pub shares: u64,
}
