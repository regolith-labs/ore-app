use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use steel::Pubkey;

use crate::gateway::{GatewayError, GatewayResult};
use crate::solana::spl_token::amount_to_ui_amount;

use super::{use_boost_wss, use_liquidity_pair, use_ore_price, OrePrice};

pub fn use_boost_tvl(mint_address: Pubkey) -> Memo<GatewayResult<f64>> {
    let boost = use_boost_wss(mint_address);
    let liquidity_pair = use_liquidity_pair(mint_address);
    let ore_price = use_ore_price();

    use_memo(move || {
        let Ok(boost) = boost.cloned() else {
            return Err(GatewayError::Unknown);
        };
        if mint_address == MINT_ADDRESS {
            // ORE case
            let Some(ore_price_f64) = ore_price.cloned() else {
                return Err(GatewayError::Unknown);
            };
            let total_deposits_f64 = amount_to_ui_amount(boost.total_deposits, TOKEN_DECIMALS);
            return Ok(ore_price_f64 * total_deposits_f64);
        } else {
            // LP case
            let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
                return Err(GatewayError::Unknown);
            };
            return Ok(liquidity_pair.total_value_usd);
        }
    })
}
