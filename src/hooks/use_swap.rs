use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    JupiterSwapApiClient,
};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::GatewayResult;

const API_URL: &str = "https://quote-api.jup.ag/v6";

pub async fn quote(
    amount: String,
    input_decimals: &u8,
    input_mint: &Pubkey,
    output_mint: &Pubkey,
    slippage_bps: u16,
) -> GatewayResult<QuoteResponse> {
    // parse input amount
    let float: f64 = amount.parse::<f64>()?;
    let scalar = 10u64.pow(*input_decimals as u32) as f64;
    let amount = (float * scalar) as u64;
    let client = JupiterSwapApiClient::new(API_URL.to_string());
    let request = QuoteRequest {
        amount,
        input_mint: *input_mint,
        output_mint: *output_mint,
        slippage_bps,
        ..QuoteRequest::default()
    };
    let response = client.quote(&request).await?;
    log::info!("quote: {:?}", response);
    Ok(response)
}
