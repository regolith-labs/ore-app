use dioxus::prelude::*;
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;

use crate::config::Token;
use crate::gateway::GatewayResult;
use crate::solana::spl_token::amount_to_ui_amount;

const API_URL: &str = "https://quote-api.jup.ag/v6";

#[derive(Debug, Clone, PartialEq)]
pub struct OrePrice(pub f64);

pub(crate) fn use_ore_price_provider() {
    let usdc = Token::usdc();
    let ore_quote = use_ore_quote(usdc.mint);
    let ore_price = use_memo(move || {
        let Some(Ok(quote)) = ore_quote.cloned() else {
            return None;
        };
        Some(OrePrice(amount_to_ui_amount(
            quote.out_amount,
            usdc.decimals,
        )))
    });
    use_context_provider(move || ore_price);
}

pub fn use_ore_price() -> Memo<Option<OrePrice>> {
    use_context()
}

pub fn use_ore_quote(output_token: Pubkey) -> Resource<GatewayResult<QuoteResponse>> {
    use_resource(move || async move {
        let client = JupiterSwapApiClient::new(API_URL.to_string());
        let request = QuoteRequest {
            amount: ore_api::consts::ONE_ORE,
            input_mint: ore_api::consts::MINT_ADDRESS,
            output_mint: output_token,
            slippage_bps: 500,
            ..QuoteRequest::default()
        };
        let response = client.quote(&request).await?;
        Ok(response)
    })
}

