use std::collections::HashMap;

use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use ore_boost_api::{consts::DENOMINATOR_BPS, state::boost_pda};
use steel::Pubkey;

use crate::{
    config::LISTED_BOOSTS,
    gateway::{ore::OreGateway, GatewayError, GatewayResult},
    hooks::use_gateway,
};

use super::{
    use_boost_config_wss, use_boost_tvl, use_boost_wss, use_liquidity_pair, use_ore_price, OrePrice,
};

pub type BoostYield = f64;

pub(crate) fn use_boost_yield_provider() {
    // Hashmap to cache resources
    let mut boosts = HashMap::new();

    // Idle ORE boost
    let boost_address = boost_pda(MINT_ADDRESS).0;
    boosts.insert(boost_address, use_boost_yield_resource(boost_address));

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        let boost_address = boost_pda(boost_meta.lp_mint).0;
        boosts.insert(boost_address, use_boost_yield_resource(boost_address));
    }

    // Setup context provider
    use_context_provider(|| boosts);
}

fn use_boost_yield_resource(address: Pubkey) -> Resource<GatewayResult<BoostYield>> {
    use_resource(move || async move {
        use_gateway()
            .get_boost_yield_7d(address)
            .await
            .map_err(GatewayError::from)
    })
}

pub fn _use_boost_yield(mint_address: Pubkey) -> Resource<GatewayResult<BoostYield>> {
    let boost_yields: HashMap<Pubkey, Resource<GatewayResult<BoostYield>>> = use_context();
    let boost_address = boost_pda(mint_address).0;
    if let Some(boost_yield) = boost_yields.get(&boost_address) {
        *boost_yield
    } else {
        use_boost_yield_resource(boost_address)
    }
}

pub fn use_boost_apr(mint_address: Pubkey) -> Memo<GatewayResult<f64>> {
    // let config = use_config();
    let boost_config = use_boost_config_wss();
    // let boost_yield = use_boost_yield(mint_address);
    let boost_tvl = use_boost_tvl(mint_address);
    let boost = use_boost_wss(mint_address);
    let liquidity_pair = use_liquidity_pair(mint_address);
    let ore_price = use_ore_price();

    let total_weight = use_memo(move || {
        let mut total_weight = 0;
        if let Ok(boost_config) = boost_config.cloned() {
            total_weight = boost_config.total_weight;
        }
        total_weight
    });

    let take_rate = use_memo(move || {
        if let Ok(boost_config) = boost_config.cloned() {
            boost_config.take_rate
        } else {
            0
        }
    });

    let boost_yield = use_memo(move || {
        if let Ok(boost) = boost.cloned() {
            let boost_take_rate = 1440.0 * *take_rate.read() as f64 / DENOMINATOR_BPS as f64;
            boost_take_rate * boost.weight as f64 / *total_weight.read() as f64
        } else {
            0.0
        }
    });

    use_memo(move || {
        // let Some(Ok(boost_yield)) = boost_yield.cloned() else {
        //     return Err(GatewayError::Unknown);
        // };

        let Ok(boost_tvl) = boost_tvl.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Ok(boost) = boost.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(ore_price_f64) = ore_price.cloned() else {
            return Err(GatewayError::Unknown);
        };

        // Adjust TVL for boost deposits
        let boost_tvl = if mint_address == MINT_ADDRESS {
            boost_tvl
        } else {
            let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
                return Err(GatewayError::Unknown);
            };
            let deposited_shares_pct = boost.total_deposits as f64 / liquidity_pair.shares as f64;
            boost_tvl * deposited_shares_pct
        };

        let apr = ((*boost_yield.read() * ore_price_f64) / boost_tvl) * 365.0 * 100.0;
        Ok(apr)
    })
}
