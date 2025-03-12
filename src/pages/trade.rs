use dioxus::prelude::*;

use crate::{
    components::*,
    config::{Token, LISTED_TOKENS_BY_TICKER},
    route::Route,
};

// Update URL with token pair
fn update_token_pair_url(navigator: &Navigator, buy: Option<Token>, sell: Option<Token>) {
    if let (Some(buy), Some(sell)) = (buy, sell) {
        // Format URL with sell token first, then buy token
        let token_pair = format!("{}-{}", sell.ticker, buy.ticker);
        navigator.replace(Route::TradeWithPair { token_pair });
    }
}

// TODO Price chart component
// TODO Activity component
#[component]
pub fn Trade(token_pair: Option<String>) -> Element {
    let navigator = use_navigator();

    // Default tokens (SOL-ORE)
    let mut buy_token = use_signal(|| Some(Token::ore()));
    let mut sell_token = use_signal(|| Some(Token::sol()));

    // Parse token pair from URL if included
    use_memo(move || {
        if let Some(pair) = &token_pair {
            // Parse the token pair in URL
            let parts: Vec<&str> = pair.split('-').collect();
            if parts.len() == 2 {
                let sell_ticker = parts[0];
                let buy_ticker = parts[1];

                // Get tokens if they exist in our list
                let found_sell = LISTED_TOKENS_BY_TICKER.get(sell_ticker).cloned();
                let found_buy = LISTED_TOKENS_BY_TICKER.get(buy_ticker).cloned();

                // Only update if both tokens were found, otherwise use defaults
                if found_sell.is_some() && found_buy.is_some() {
                    sell_token.set(found_sell);
                    buy_token.set(found_buy);
                } else {
                    // If either token is invalid, update URL to default pair
                    update_token_pair_url(&navigator, Some(Token::ore()), Some(Token::sol()));
                }
            }
        } else {
            // If we're on the /trade route without token pair, update the URL to include the default token pair
            update_token_pair_url(
                &navigator,
                buy_token.peek().clone(),
                sell_token.peek().clone(),
            );
        }
    });

    let update_url = EventHandler::new(move |(buy, sell): (Option<Token>, Option<Token>)| {
        update_token_pair_url(&navigator, buy, sell);
    });

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Trade",
                subtitle: "Swap tokens at the best price."
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
    rsx! {
        Trade {
            token_pair: Some(token_pair)
        }
    }
}
