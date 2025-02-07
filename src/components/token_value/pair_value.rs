use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;

use crate::{components::{token_value::{utils::TokenValueSize, OreValue, TokenValueSmall}, Col}, utils::LiquidityPair};

#[component]
pub fn LiquidityPairValue(class: Option<String>, liquidity_pair: LiquidityPair, with_decimal_units: Option<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let (ore_amount_f64, token_amount_f64, token_ticker) = if liquidity_pair.token_a.ticker == "ORE" {
        (liquidity_pair.balance_a_f64, liquidity_pair.balance_b_f64, liquidity_pair.token_b.ticker.clone())
    } else {
        (liquidity_pair.balance_b_f64, liquidity_pair.balance_a_f64, liquidity_pair.token_a.ticker.clone())
    };
    rsx! {
        Col {
            class: "gap-2 {class}",
            OreValue {
                class: "ml-auto",
                ui_amount_string: ore_amount_f64.to_string(),
                with_decimal_units: true,
                size: TokenValueSize::Small,
            }
            TokenValueSmall {
                class: "ml-auto",
                amount: token_amount_f64.to_string(),
                ticker: token_ticker.clone(),
                with_decimal_units: with_decimal_units,
            }
        }
    }
}


#[component]
pub fn LiquidityPairStakeValue(
    class: Option<String>, 
    stake_balance: u64, 
    liquidity_pair: LiquidityPair, 
    with_decimal_units: Option<bool>
) -> Element {
    let class = class.unwrap_or("".to_string());
    let (ore_amount_f64, token_amount_f64, token_ticker, token_decimals) = liquidity_pair.get_stake_amounts(stake_balance);
    rsx! {
        Col {
            class: "gap-2 {class}",
            OreValue {
                class: "ml-auto",
                ui_amount_string: format!("{:.1$}", ore_amount_f64, TOKEN_DECIMALS as usize),
                with_decimal_units: true,
                size: TokenValueSize::Small,
            }
            TokenValueSmall {
                class: "ml-auto",
                amount: format!("{:.1$}", token_amount_f64, token_decimals as usize),
                ticker: token_ticker.clone(),
                with_decimal_units: with_decimal_units,
            }
        }
    }
}

