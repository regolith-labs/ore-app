use dioxus::prelude::*;
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::time::Duration;

use crate::config::{Token, LISTED_TOKENS};
use crate::gateway::{GatewayError, GatewayResult, UiTokenAmount};
use crate::hooks::{use_token_balance, use_wallet, Wallet};
use crate::solana::spl_token::amount_to_ui_amount;

const API_URL: &str = "https://quote-api.jup.ag/v6";
const REFRESH_INTERVAL_SECS: u64 = 300; // 5 minutes

#[derive(Debug, Clone, PartialEq)]
pub struct TokenPrice(pub f64);

pub(crate) fn use_token_price_provider() {
    // Create a signal for storing raw price resource results
    let price_resources =
        use_signal(|| HashMap::<Pubkey, Resource<GatewayResult<TokenPrice>>>::new());

    // Create a memo that derives token prices from resources (like use_ore_price)
    let token_prices = use_memo(move || {
        let mut prices = HashMap::new();

        // Convert resources to simple token prices
        for (mint, resource) in price_resources.read().iter() {
            if let Some(Ok(price)) = resource.read().as_ref() {
                prices.insert(*mint, price.clone());
            }
        }

        prices
    });

    // Initialize prices and setup periodic refresh for tokens with balance
    setup_token_prices(price_resources.clone());

    // Provide the memo as context
    use_context_provider(|| token_prices);
}

// Setup token prices with initialization and periodic refresh
fn setup_token_prices(
    mut price_resources: Signal<HashMap<Pubkey, Resource<GatewayResult<TokenPrice>>>>,
) {
    use_effect(move || {
        let wallet = use_wallet();
        let mut price_resources_clone = price_resources.clone();

        spawn(async move {
            // Initial fetch for tokens with balance
            if matches!(*wallet.read(), Wallet::Connected(_)) {
                // Get tokens with balance
                let tokens_with_balance = get_tokens_with_balance();

                // Always include SOL and USDC as they're commonly used
                let mut tokens_to_initialize = tokens_with_balance;
                tokens_to_initialize.extend([Token::sol().mint, Token::usdc().mint]);

                // Deduplicate
                tokens_to_initialize.sort();
                tokens_to_initialize.dedup();

                // Initialize prices
                if !tokens_to_initialize.is_empty() {
                    log::info!(
                        "Initializing prices for {} tokens with balance",
                        tokens_to_initialize.len()
                    );

                    for mint in tokens_to_initialize {
                        let resource = fetch_token_price_resource(mint, None);
                        price_resources.write().insert(mint, resource);
                    }
                }
            }

            // Wait a bit before starting the refresh cycle
            async_std::task::sleep(Duration::from_secs(5)).await;

            // Periodic refresh loop
            loop {
                if matches!(*wallet.read(), Wallet::Connected(_)) {
                    // Get tokens with balance
                    let tokens_to_update = get_tokens_with_balance();

                    // Always include SOL and USDC
                    let mut all_tokens_to_update = tokens_to_update;
                    // all_tokens_to_update.extend([Token::sol().mint, Token::usdc().mint]);

                    // Deduplicate
                    all_tokens_to_update.sort();
                    all_tokens_to_update.dedup();

                    // Update resources
                    if !all_tokens_to_update.is_empty() {
                        log::info!(
                            "Refreshing prices for {} tokens",
                            all_tokens_to_update.len()
                        );

                        for mint in all_tokens_to_update {
                            let price_map = &mut price_resources_clone.write();

                            if let Some(resource) = price_map.get_mut(&mint) {
                                resource.restart();
                            } else {
                                let new_resource = fetch_token_price_resource(mint, None);
                                price_map.insert(mint, new_resource);
                            }

                            // Drop the price_map write guard before the next iteration
                            drop(price_map);
                        }
                    }
                }

                // Sleep until next refresh
                async_std::task::sleep(Duration::from_secs(REFRESH_INTERVAL_SECS)).await;
            }
        });
    });
}

// Helper to get tokens with balance > 0
fn get_tokens_with_balance() -> Vec<Pubkey> {
    let token_list: Vec<Token> = LISTED_TOKENS.values().cloned().collect();
    let mut tokens_with_balance = Vec::new();

    for token in token_list {
        let balance = use_token_balance(token.mint);
        let has_balance = match balance.read().as_ref() {
            Some(Ok(amount)) => amount.ui_amount.map_or(false, |ui| ui > 0.0),
            _ => false,
        };

        if has_balance {
            tokens_with_balance.push(token.mint);
        }
    }

    tokens_with_balance
}

// Helper function to fetch token price with amount
fn fetch_token_price_resource(
    mint: Pubkey,
    amount_f64: Option<f64>,
) -> Resource<GatewayResult<TokenPrice>> {
    let usdc = Token::usdc();

    use_resource(move || async move {
        let client = JupiterSwapApiClient::new(API_URL.to_string());

        // Find token in our list to get decimals
        let token = match LISTED_TOKENS.get(&mint) {
            Some(token) => token,
            None => return Err(GatewayError::AccountNotFound),
        };

        // If amount provided, use it, otherwise use 1 token as default
        let raw_amount = if let Some(amount) = amount_f64 {
            (amount * 10f64.powi(token.decimals as i32)) as u64
        } else {
            10u64.pow(token.decimals as u32) // 1 token as default
        };

        let request = QuoteRequest {
            amount: raw_amount,
            input_mint: mint,
            output_mint: usdc.mint,
            slippage_bps: 500,
            ..QuoteRequest::default()
        };

        let response = client.quote(&request).await?;

        // Calculate price per token
        let quote_amount = amount_to_ui_amount(response.out_amount, usdc.decimals);
        let input_amount = amount_f64.unwrap_or(1.0);
        let price_per_token = quote_amount / input_amount;

        Ok(TokenPrice(price_per_token))
    })
}

// Hook to get a token price from context - now returns Option<TokenPrice> directly
pub fn use_token_price(mint: Pubkey) -> Option<TokenPrice> {
    // Get the memo from context
    let token_prices: Memo<HashMap<Pubkey, TokenPrice>> = use_context();
    let prices = token_prices.read();
    prices.get(&mint).cloned()
}

// Legacy hook to support backward compatibility
pub fn use_token_price_with_amount(
    mint: Pubkey,
    amount_f64: Option<f64>,
) -> Resource<GatewayResult<TokenPrice>> {
    // Simply create a new resource
    fetch_token_price_resource(mint, amount_f64)
}
