use dioxus::prelude::*;
use jupiter_swap_api_client::quote::QuoteRequest;
use jupiter_swap_api_client::JupiterSwapApiClient;
use std::time::Duration;

use crate::config::{Token, LISTED_TOKENS};
use crate::hooks::{use_token_balance, use_wallet, Wallet};
use crate::solana::spl_token::amount_to_ui_amount;

const API_URL: &str = "https://quote-api.jup.ag/v6";
const REFRESH_INTERVAL_SECS: u64 = 300; // 5 minutes

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithValue {
    pub token: Token,
    pub balance: f64,
    pub price_per_token: f64,
    pub total_value: f64,
}

// Main provider for token values
pub(crate) fn use_token_price_provider() {
    // Create a signal for storing token values
    let token_values = use_signal(Vec::<TokenWithValue>::new);

    use_effect(move || {
        let mut token_values = token_values.clone();

        // Initial fetch with simple retries
        spawn(async move {
            const MAX_RETRIES: usize = 10;
            const RETRY_DELAY_MS: u64 = 1000;

            // Try multiple times to get token balances if wallet is connected
            for _ in 0..MAX_RETRIES {
                // Check wallet connection first
                if !matches!(*use_wallet().read(), Wallet::Connected(_)) {
                    async_std::task::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                    continue;
                }

                // Only proceed with balance fetching if wallet is connected
                let tokens = get_tokens_with_balance();

                if !tokens.is_empty() {
                    let values = fetch_token_values(&tokens).await;
                    token_values.set(values);
                    break; // Exit retry loop on success
                } else {
                    async_std::task::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }

            // Continue with periodic refresh
            loop {
                async_std::task::sleep(Duration::from_secs(REFRESH_INTERVAL_SECS)).await;

                if matches!(*use_wallet().read(), Wallet::Connected(_)) {
                    let tokens = get_tokens_with_balance();

                    if !tokens.is_empty() {
                        let values = fetch_token_values(&tokens).await;
                        token_values.set(values);
                    }
                }
            }
        });
    });

    use_context_provider(|| token_values);
}

// Unified hook for token prices
pub fn use_tokens_with_values() -> Vec<TokenWithValue> {
    let token_values: Signal<Vec<TokenWithValue>> = use_context();
    let values = token_values.cloned();
    values
}

// Helper to get tokens with balance > 0
fn get_tokens_with_balance() -> Vec<(Token, f64)> {
    let mut tokens_with_balance = Vec::new();

    // Get token list
    let token_list: Vec<Token> = LISTED_TOKENS.values().cloned().collect();

    //Go through tokens and find those with balance > 0
    for token in token_list {
        let balance = use_token_balance(token.mint);

        match balance.cloned() {
            Some(Ok(amount)) => {
                if let Some(ui_amount) = amount.ui_amount {
                    if ui_amount > 0.0 {
                        tokens_with_balance.push((token, ui_amount));
                    }
                }
            }
            _ => continue,
        }
    }
    tokens_with_balance
}

// Get quote from jupiter + create price
async fn fetch_token_values(tokens: &[(Token, f64)]) -> Vec<TokenWithValue> {
    let client = JupiterSwapApiClient::new(API_URL.to_string());
    let usdc = Token::usdc();
    let mut results = Vec::new();

    for (token, amount) in tokens {
        // Special case for USDC - the price is exactly 1.0 by definition
        if token.mint == usdc.mint {
            results.push(TokenWithValue {
                token: token.clone(),
                balance: *amount,
                price_per_token: 1.0,
                total_value: *amount, // 1.0 * amount
            });
            continue;
        }

        let raw_amount = (*amount * 10f64.powi(token.decimals as i32)) as u64;
        let request = QuoteRequest {
            amount: raw_amount,
            input_mint: token.mint,
            output_mint: usdc.mint,
            slippage_bps: 500,
            ..QuoteRequest::default()
        };

        match client.quote(&request).await {
            Ok(response) => {
                let quote_amount = amount_to_ui_amount(response.out_amount, usdc.decimals);
                let price = quote_amount / amount;
                results.push(TokenWithValue {
                    token: token.clone(),
                    balance: *amount,
                    price_per_token: price,
                    total_value: price * (*amount),
                });
            }
            Err(e) => {
                log::error!("Failed to fetch price for {}: {:?}", token.ticker, e);
            }
        }
    }

    results
}
