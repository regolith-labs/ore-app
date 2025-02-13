use std::collections::HashMap;

use dioxus::prelude::*;
use steel::Pubkey;

use crate::{config::{LpType, LISTED_BOOSTS, LISTED_BOOSTS_BY_MINT, LISTED_TOKENS, LISTED_TOKENS_BY_TICKER}, gateway::{kamino::KaminoGateway, meteora::MeteoraGateway, GatewayError, GatewayResult, Rpc}, hooks::use_gateway, utils::LiquidityPair};



pub(crate) fn use_liquidity_pairs_provider() {
    // Hashmap to cache resources
    let mut liquidity_pairs = HashMap::new();

    // Listed liquidity pairs
    for boost_meta in LISTED_BOOSTS.iter() {
        liquidity_pairs.insert(boost_meta.lp_mint, use_liquidity_pair_resource(boost_meta.lp_mint));
    }

    // Setup context provider
    use_context_provider(|| liquidity_pairs);
}

fn use_liquidity_pair_resource(lp_mint_address: Pubkey) -> Resource<GatewayResult<LiquidityPair>> {
    use_resource(move || async move {
        let Some(boost_meta) = LISTED_BOOSTS_BY_MINT.get(&lp_mint_address) else {
            return Err(GatewayError::Unknown);
        };
        let lp_type: LpType = boost_meta.lp_type;
        let lp_mint_supply = use_gateway().rpc.get_token_supply(&boost_meta.lp_mint).await?;
        match lp_type {
            LpType::Kamino => {
                let strategy_metrics = use_gateway().get_kamino_strategy_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS_BY_TICKER.get(&strategy_metrics.token_a).unwrap();
                let token_b = LISTED_TOKENS_BY_TICKER.get(&strategy_metrics.token_b).unwrap();
                return Ok(LiquidityPair {
                    token_a: token_a.clone(),
                    token_b: token_b.clone(),
                    balance_a_f64: strategy_metrics.vault_balances.token_a.total,
                    balance_b_f64: strategy_metrics.vault_balances.token_b.total,
                    total_value_usd: strategy_metrics.total_value_locked,
                    shares: lp_mint_supply.amount.parse::<u64>().unwrap_or(0),
                });
            }
            LpType::Meteora => {
                let pool_metrics = use_gateway().get_meteora_pool_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS.get(&pool_metrics.pool_token_mints[0]).unwrap();
                let token_b = LISTED_TOKENS.get(&pool_metrics.pool_token_mints[1]).unwrap();
                let balance_a = pool_metrics.pool_token_amounts[0];
                let balance_b = pool_metrics.pool_token_amounts[1];
                return Ok(LiquidityPair {
                    token_a: token_a.clone(),
                    token_b: token_b.clone(),
                    balance_a_f64: balance_a,
                    balance_b_f64: balance_b,
                    total_value_usd: pool_metrics.pool_tvl,
                    shares: lp_mint_supply.amount.parse::<u64>().unwrap_or(0),
                });
            }
        }
    })
}


pub fn use_liquidity_pair(lp_mint_address: Pubkey) -> Resource<GatewayResult<LiquidityPair>> {
    let liquidity_pairs: HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>> = use_context();
    if let Some(liquidity_pair) = liquidity_pairs.get(&lp_mint_address) {
        *liquidity_pair
    } else {
        use_liquidity_pair_resource(lp_mint_address)
    }
}

pub fn use_all_liquidity_pairs() -> HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>> {
    use_context()
}
