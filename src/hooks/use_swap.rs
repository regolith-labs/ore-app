use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_debounce, UseDebounce};
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    swap::SwapRequest,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, transaction::VersionedTransaction};

use crate::gateway::{GatewayError, GatewayResult};

use super::{use_wallet, Asset, GetPubkey};

const API_URL: &str = "https://quote-api.jup.ag/v6";

pub fn use_swap(
    quote: Signal<Option<QuoteResponse>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || {
        let client = JupiterSwapApiClient::new(API_URL.to_string());
        async move {
            let quote = &*quote.read();
            let pubkey = wallet.get_pubkey()?;
            match quote {
                Some(quote) => {
                    let request = SwapRequest {
                        user_public_key: pubkey,
                        quote_response: quote.clone(),
                        config: TransactionConfig::default(),
                    };
                    let response = client.swap(&request, None).await?;
                    log::info!("swap resp: {:?}", response);
                    let vtx: VersionedTransaction = bincode::deserialize(
                        response.swap_transaction.as_slice(),
                    )
                    .map_err(|err| {
                        log::error!("{:?}", err);
                        GatewayError::FailedDeserialization
                    })?;
                    log::info!("vtx: {:?}", vtx);
                    Ok(vtx)
                }
                None => Err(GatewayError::JupSwapError),
            }
        }
    })
}

pub fn use_quote(
    input_token: Signal<Asset>,
    mut input_token_amount: Signal<String>,
    output_token: Signal<Asset>,
    mut output_token_amount: Signal<String>,
    mut quote_response: Signal<Option<QuoteResponse>>,
) -> UseDebounce<String> {
    use_debounce::<String>(std::time::Duration::from_millis(750), move |input_str| {
        spawn(async move {
            let mut clear = false;
            if let Ok(float) = input_str.parse::<f64>() {
                if float == 0f64 {
                    clear = true;
                } else {
                    let input_token = &*input_token.read();
                    let input_token = input_token.clone();
                    let output_token = &*output_token.read();
                    let output_token = output_token.clone();
                    match quote(
                        float,
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
                            quote_response.set(Some(quote));
                        }
                        Err(err) => {
                            clear = true;
                            log::error!("{:?}", err);
                        }
                    };
                }
            } else {
                clear = true;
            }
            if clear {
                input_token_amount.set("0.0".to_string());
                output_token_amount.set("0.0".to_string());
                quote_response.set(None);
            }
        });
    })
}

async fn quote(
    float: f64,
    input_decimals: &u8,
    input_mint: &Pubkey,
    output_mint: &Pubkey,
    slippage_bps: u16,
) -> GatewayResult<QuoteResponse> {
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
