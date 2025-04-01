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
use crate::hooks::{use_wallet, use_gateway, GetPubkey}; // Explicitly import use_gateway

const API_URL: &str = "https://quote-api.jup.ag/v6";

pub fn use_swap_transaction(
    quote: Signal<Option<QuoteResponse>>,
    sell_token: Signal<Option<Token>>,
    sell_token_balance: Signal<GatewayResult<UiTokenAmount>>,
    priority_fee: Signal<u64>,
    mut err: Signal<Option<TokenInputError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let gateway = use_gateway();
    use_resource(move || {
        let client = JupiterSwapApiClient::new(API_URL.to_string());
        async move {
            let pubkey = wallet.pubkey()?;
            let Some(quote) = quote.cloned() else {
                return Err(GatewayError::JupSwapError);
            };
            let Some(sell_token) = sell_token.read().clone() else {
                return Err(GatewayError::Unknown);
            };
            let Ok(sell_token_balance) = sell_token_balance.cloned() else {
                err.set(Some(TokenInputError::InsufficientBalance(sell_token.clone())));
                return Err(GatewayError::Unknown);
            };

            let sell_token_balance_u64 = sell_token_balance
                .amount
                .parse::<u64>()
                .map_err(|_| GatewayError::Unknown)?;
            let sell_token_amount_u64 = quote.in_amount;
            if sell_token_balance_u64 < sell_token_amount_u64 {
                err.set(Some(TokenInputError::InsufficientBalance(sell_token.clone())));
                return Err(GatewayError::Unknown);
            };

            let temp_request = SwapRequest {
                user_public_key: pubkey,
                quote_response: quote.clone(),
                config: TransactionConfig::default(),
            };
            let temp_response = client.swap(&temp_request, None).await?;
            let temp_vtx: VersionedTransaction =
                bincode::deserialize(&temp_response.swap_transaction).map_err(|err| {
                    log::error!("Deserialization failed: {:?}", err);
                    GatewayError::FailedDeserialization
                })?;

            let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&temp_vtx).await {
                Ok(fee) => fee,
                Err(e) => {
                    log::warn!("Failed to fetch priority fee estimate: {:?}, using fallback {}", e, *priority_fee.read());
                    *priority_fee.read()
                }
            };

            let mut config = TransactionConfig::default();
            config.compute_unit_price_micro_lamports = Some(dynamic_priority_fee);
            let request = SwapRequest {
                user_public_key: pubkey,
                quote_response: quote,
                config,
            };
            let response = client.swap(&request, None).await?;
            let vtx: VersionedTransaction =
                bincode::deserialize(&response.swap_transaction).map_err(|err| {
                    log::error!("Deserialization failed: {:?}", err);
                    GatewayError::FailedDeserialization
                })?;

            Ok(vtx)
        }
    })
}