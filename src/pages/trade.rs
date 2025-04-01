use dioxus::prelude::*;

use crate::{
    components::*,
    config::{Token, LISTED_TOKENS_BY_TICKER},
    route::Route,
};

// TODO Price chart component
// TODO Activity component
#[component]
pub fn Trade(token_pair: Option<String>) -> Element {
    let navigator = use_navigator();

    // Default tokens (SOL-ORE)
    let mut buy_token = use_signal(|| Some(Token::ore()));
    let mut sell_token = use_signal(|| Some(Token::sol()));

    // Parse token pair from URL if included
    use_effect(move || {
        if let Some(pair) = &token_pair {
            if let Some((sell, buy)) = parse_token_pair(pair) {
                sell_token.set(Some(sell));
                buy_token.set(Some(buy));
            } else {
                // Invalid pair, redirect to default
                update_token_pair_url(&navigator, Some(Token::ore()), Some(Token::sol()));
            }
        } else {
            // If we're on the /trade route without token pair, update the URL to include the default token pair
            update_token_pair_url(&navigator, buy_token.cloned(), sell_token.cloned());
        }
    });

    let update_url = EventHandler::new(move |(buy, sell): (Option<Token>, Option<Token>)| {
        update_token_pair_url(&navigator, buy, sell);
    });

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Row {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8 justify-between",
                Heading {
                    class: "w-full",
                    title: "Trade",
                    subtitle: "Swap tokens at the best price."
                }
                DocsButton {
                    tab: DocsTab::Tokenomics
                }
            }
            SwapForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                buy_token,
                sell_token,
                on_tokens_change: update_url,
            }
        }
    }
}

#[component]
pub fn TradeWithPair(token_pair: String) -> Element {
    // Determine if URL contains valid pair
    let valid_pair = parse_token_pair(&token_pair).is_some();

    // If valid, pass the token pair, otherwise default redirect to /trade/SOL-ORE
    rsx! {
        Trade {
            token_pair: if valid_pair { Some(token_pair) } else { None }
        }
    }
}

// Parse and validate a token pair string SELL-BUY
fn parse_token_pair(token_pair: &str) -> Option<(Token, Token)> {
    let parts: Vec<&str> = token_pair.split('-').collect();
    if parts.len() == 2 {
        let sell_ticker = parts[0];
        let buy_ticker = parts[1];

        // Get tokens if they exist in our listed tokens
        let found_sell = LISTED_TOKENS_BY_TICKER.get(sell_ticker).cloned();
        let found_buy = LISTED_TOKENS_BY_TICKER.get(buy_ticker).cloned();

        // Return the pair if both tokens were found
        if let (Some(sell), Some(buy)) = (found_sell, found_buy) {
            return Some((sell, buy));
        }
    }
    None
}

// Update URL with token pair
fn update_token_pair_url(navigator: &Navigator, buy: Option<Token>, sell: Option<Token>) {
    if let (Some(buy), Some(sell)) = (buy, sell) {
        // Format URL with SELL-BUY
        let token_pair = format!("{}-{}", sell.ticker, buy.ticker);
        navigator.replace(Route::TradeWithPair { token_pair });
    }
}
