use dioxus::prelude::*;
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;

use crate::config::{Token, LISTED_TOKENS};
use crate::gateway::{GatewayError, GatewayResult};
use crate::solana::spl_token::amount_to_ui_amount;

const API_URL: &str = "https://quote-api.jup.ag/v6";

#[derive(Debug, Clone, PartialEq)]
pub struct TokenPrice(pub f64);

// Simple token price hook - gets the price for a single token
pub fn use_token_price(mint: Pubkey) -> Resource<GatewayResult<TokenPrice>> {
    // Get the USDC token for quoting
    let usdc = Token::usdc();

    // Create a resource to fetch the price
    use_resource(move || async move {
        let client = JupiterSwapApiClient::new(API_URL.to_string());

        // Find token in our list to get decimals
        let token = match LISTED_TOKENS.get(&mint) {
            Some(token) => token,
            None => return Err(GatewayError::AccountNotFound),
        };

        // Use 1 token as the input amount
        let one_token = 10u64.pow(token.decimals as u32);

        let request = QuoteRequest {
            amount: one_token,
            input_mint: mint,
            output_mint: usdc.mint,
            slippage_bps: 500,
            ..QuoteRequest::default()
        };

        let response = client.quote(&request).await?;

        // Convert to UI amount and return as TokenPrice
        let price = amount_to_ui_amount(response.out_amount, usdc.decimals);
        Ok(TokenPrice(price))
    })
}
