use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_debounce, UseDebounce};
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    swap::SwapRequest,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::{pubkey::Pubkey, transaction::VersionedTransaction};

use crate::gateway::{GatewayError, GatewayResult};
use crate::config::Token;

use crate::hooks::{use_wallet, GetPubkey};

const API_URL: &str = "https://quote-api.jup.ag/v6";

pub fn use_swap_transaction(
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
                    let vtx: VersionedTransaction = bincode::deserialize(
                        response.swap_transaction.as_slice(),
                    )
                    .map_err(|err| {
                        log::error!("{:?}", err);
                        GatewayError::FailedDeserialization
                    })?;
                    Ok(vtx)
                }
                None => Err(GatewayError::JupSwapError),
            }
        }
    })
}

pub fn use_quote(
    input_token: Signal<Token>,
    mut input_token_amount: Signal<Option<String>>,
    output_token: Signal<Token>,
    mut output_token_amount: Signal<Option<String>>,
    mut quote_response: Signal<Option<QuoteResponse>>,
) -> UseDebounce<String> {
    use_debounce::<String>(std::time::Duration::from_millis(750), move |input_str| {
        spawn(async move {
            let mut clear = false;
            if let Ok(float) = input_str.parse::<f64>() {
                if float == 0f64 {
                    clear = true;
                } else {
                    let input_token = input_token.read().clone();
                    let output_token = output_token.read().clone();
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
                            let _input_amount = input_amount / input_decimals;

                            // output amount
                            let output_amount = quote.out_amount as f64;
                            let output_decimals = 10u64.pow(output_token.decimals as u32) as f64;
                            let output_amount = output_amount / output_decimals;

                            // swap
                            // input_token_amount.set(Some(input_amount.to_string()));
                            output_token_amount.set(Some(output_amount.to_string()));
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
                input_token_amount.set(Some("".to_string()));
                output_token_amount.set(Some("".to_string()));
                quote_response.set(None);
            }
        });
    })
}

pub fn use_ore_quote(
    output_token: Pubkey,
) -> Resource<GatewayResult<QuoteResponse>> {
    use_resource(move || {
        async move {
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
        }
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
