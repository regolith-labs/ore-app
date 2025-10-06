use std::collections::HashMap;

use dioxus::prelude::*;
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    JupiterSwapApiClient,
};
use ore_api::consts::MINT_ADDRESS;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use crate::config::Token;
use crate::gateway::GatewayResult;
use crate::solana::spl_token::amount_to_ui_amount;

const PRICE_API_URL: &str = "https://lite-api.jup.ag/price/v3";

#[derive(Debug, Clone, PartialEq)]
pub struct OrePrice(pub f64);

pub fn use_ore_price() -> Memo<Option<f64>> {
    let ore_quote = use_ore_quote(MINT_ADDRESS);
    use_memo(move || {
        let Some(Ok(price)) = ore_quote.cloned() else {
            return None;
        };
        Some(price)
    })
}

pub fn use_ore_quote(output_token: Pubkey) -> Resource<GatewayResult<f64>> {
    use_resource(move || async move {
        let client = reqwest::Client::new();
        let url = format!("{}?ids={}", PRICE_API_URL, output_token.to_string());
        let asset_price = client
            .get(url)
            .send()
            .await?
            .json::<PriceResponse>()
            .await?
            .0
            .get(&output_token.to_string())
            .unwrap()
            .usd_price;
        Ok(asset_price)
    })
}

// pub fn use_ore_quote(output_token: Pubkey) -> Resource<GatewayResult<QuoteResponse>> {
//     use_resource(move || async move {
//         let client = JupiterSwapApiClient::new(API_URL.to_string());
//         let request = QuoteRequest {
//             amount: ore_api::consts::ONE_ORE,
//             input_mint: ore_api::consts::MINT_ADDRESS,
//             output_mint: output_token,
//             slippage_bps: 500,
//             ..QuoteRequest::default()
//         };
//         let response = client.quote(&request).await?;
//         Ok(response)
//     })
// }

#[derive(Debug, Deserialize, Clone)]
struct PriceResponse(HashMap<String, AssetPrice>);

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct AssetPrice {
    #[serde(rename = "usdPrice")]
    usd_price: f64,
    #[serde(rename = "blockId")]
    block_id: u64,
    #[serde(rename = "decimals")]
    decimals: u8,
    #[serde(rename = "priceChange24h")]
    price_change_24h: f64,
}
