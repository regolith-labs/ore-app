use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_debounce, UseDebounce};
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse, SwapMode},
    route_plan_with_metadata::RoutePlanWithMetadata,
    swap::SwapRequest,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use rust_decimal::Decimal;
use solana_sdk::{pubkey::Pubkey, transaction::VersionedTransaction};

use crate::config::Token;
use crate::{
    components::TokenInputError,
    gateway::{GatewayError, GatewayResult, UiTokenAmount},
};

#[cfg(not(feature = "web"))]
use super::tip_ix;
use crate::hooks::{use_wallet, GetPubkey};

const API_URL: &str = "https://quote-api.jup.ag/v6";

pub fn use_swap_transaction(
    quote: Signal<Option<QuoteResponse>>,
    sell_token: Signal<Option<Token>>,
    sell_token_balance: Signal<GatewayResult<UiTokenAmount>>,
    _priority_fee: Signal<u64>,
    mut err: Signal<Option<TokenInputError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || {
        let client = JupiterSwapApiClient::new(API_URL.to_string());
        async move {
            // Get resources
            let pubkey = wallet.pubkey()?;
            let Some(quote) = quote.cloned() else {
                return Err(GatewayError::JupSwapError);
            };
            let Some(sell_token) = sell_token.read().clone() else {
                return Err(GatewayError::Unknown);
            };
            let Ok(sell_token_balance) = sell_token_balance.cloned() else {
                err.set(Some(TokenInputError::InsufficientBalance(sell_token)));
                return Err(GatewayError::Unknown);
            };

            // Check if balance is sufficient
            let sell_token_balance_u64 = sell_token_balance
                .amount
                .parse::<u64>()
                .map_err(|_| GatewayError::Unknown)?;
            let sell_token_amount_u64 = quote.in_amount;
            if sell_token_balance_u64 < sell_token_amount_u64 {
                err.set(Some(TokenInputError::InsufficientBalance(sell_token)));
                return Err(GatewayError::Unknown);
            }

            // Build transaction from jupiter quote
            let request = SwapRequest {
                user_public_key: pubkey,
                quote_response: quote.clone(),
                config: TransactionConfig::default(),
            };
            let response = client.swap(&request, None).await?;
            let vtx: VersionedTransaction =
                bincode::deserialize(response.swap_transaction.as_slice()).map_err(|err| {
                    log::error!("{:?}", err);
                    GatewayError::FailedDeserialization
                })?;

            // Note: We don't add Jito tip here as Jupiter's API already creates a complete transaction

            Ok(vtx)
        }
    })
}

fn zero_quote(input_mint: Pubkey, output_mint: Pubkey) -> QuoteResponse {
    QuoteResponse {
        input_mint,
        in_amount: 0,
        output_mint,
        out_amount: 0,
        other_amount_threshold: 0,
        swap_mode: SwapMode::ExactIn,
        slippage_bps: 0,
        computed_auto_slippage: None,
        uses_quote_minimizing_slippage: None,
        platform_fee: None,
        price_impact_pct: Decimal::from(0),
        route_plan: RoutePlanWithMetadata::default(),
        context_slot: 0,
        time_taken: 0.0,
    }
}

pub fn use_quote(
    input_token: Signal<Option<Token>>,
    output_token: Signal<Option<Token>>,
    mut output_token_amount: Signal<String>,
    mut quote_response: Signal<Option<QuoteResponse>>,
) -> UseDebounce<String> {
    use_debounce::<String>(std::time::Duration::from_millis(750), move |input_str| {
        spawn(async move {
            let mut clear = false;

            // Get tokens
            let Some(input_token) = input_token.read().clone() else {
                return;
            };
            let Some(output_token) = output_token.read().clone() else {
                return;
            };

            // Parse input amount
            if let Ok(float) = input_str.parse::<f64>() {
                if float == 0f64 {
                    output_token_amount.set("0".to_string());
                    quote_response.set(Some(zero_quote(input_token.mint, output_token.mint)));
                    return;
                } else {
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
                output_token_amount.set("".to_string());
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
