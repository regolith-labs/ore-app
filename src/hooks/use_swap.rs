use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_debounce, UseDebounce};
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    JupiterSwapApiClient,
};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::GatewayResult;

use super::Asset;

const API_URL: &str = "https://quote-api.jup.ag/v6";

pub fn use_quote(
    input_token: Signal<Asset>,
    mut input_token_amount: Signal<String>,
    output_token: Signal<Asset>,
    mut output_token_amount: Signal<String>,
) -> UseDebounce<String> {
    use_debounce::<String>(std::time::Duration::from_millis(750), move |input_str| {
        spawn({
            async move {
                let input_token = &*input_token.read();
                let output_token = &*output_token.read();
                match quote(
                    input_str,
                    &input_token.decimals,
                    &input_token.mint,
                    &output_token.mint,
                    500,
                )
                .await
                {
                    Ok(quote) => {
                        // input amount
                        let input_amount = quote.in_amount as f64;
                        let input_decimals = 10u64.pow(input_token.decimals as u32) as f64;
                        let input_amount = input_amount / input_decimals;
                        // output amount
                        let output_amount = quote.out_amount as f64;
                        let output_decimals = 10u64.pow(output_token.decimals as u32) as f64;
                        let output_amount = output_amount / output_decimals;
                        // swap
                        input_token_amount.set(input_amount.to_string());
                        output_token_amount.set(output_amount.to_string());
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                };
            }
        });
    })
}

async fn quote(
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
    Ok(response)
}
