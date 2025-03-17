use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

use crate::{
    components::*,
    config::{Token, LISTED_TOKENS},
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{
        use_ore_price, use_token_balance, use_token_price, use_token_price_with_amount, use_wallet,
        OrePrice, TokenPrice, Wallet,
    },
    route::Route,
};

pub fn TokenList() -> Element {
    let token_list: Vec<Token> = LISTED_TOKENS.values().cloned().collect();

    // Get all token balances at top level
    let mut all_balances = HashMap::new();
    for token in token_list.iter() {
        let balance = use_token_balance(token.mint);
        all_balances.insert(token.mint, balance);
    }

    // Filter tokens with balance > 0
    let tokens_with_balance = use_memo(move || {
        token_list
            .iter()
            .filter_map(|token| {
                let balance = all_balances.get(&token.mint).unwrap();

                if let Some(Ok(amount)) = balance.read().as_ref() {
                    if let Some(ui_amount) = amount.ui_amount {
                        if ui_amount > 0.0 {
                            return Some((token.clone(), ui_amount));
                        }
                    }
                }
                None
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        Col {
            class: "w-full",
            {
                tokens_with_balance.read().iter().map(|(token, ui_amount)| {
                    // Price fetching logic (only for tokens with balance > 0)
                    let price_display = {
                        // Use the global token price resource for all tokens
                        let token_price = use_token_price(token.mint);

                        // Then use the value in the match (now simpler with Option<TokenPrice>)
                        match token_price {
                            Some(price) => {
                                // Calculate the total value of user's tokens (price * balance)
                                let total_value = price.0 * ui_amount;
                                rsx! {
                                    span {
                                        class: "font-medium text-elements-highEmphasis",
                                        "${total_value:.2}"
                                    }
                                }
                            },
                            None => rsx! {
                                div {
                                    class: "h-5 w-16 rounded-md animate-pulse bg-surface-primary/60 my-auto",
                                }
                            }
                        }
                    };

                    // Render token row with price
                    rsx! {
                        Link {
                            to: crate::route::Route::TransferWithToken { token_ticker: token.ticker.clone() },
                            Row {
                                key: "{token.mint}",
                                class: "w-full justify-between items-center py-4 px-4 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
                                // Left section with icon and token info
                                Row {
                                    class: "items-center",
                                    gap: 4,
                                    img {
                                        class: "w-8 h-8 rounded-full shrink-0",
                                        src: "{token.image}",
                                    }
                                    Col {
                                        span {
                                            class: "font-medium text-elements-highEmphasis",
                                            "{token.name}"
                                        }
                                        span {
                                            class: "font-medium text-xs text-elements-lowEmphasis",
                                            {format!("{:.4} {}", ui_amount, token.ticker)}
                                        }
                                    }
                                }
                                // Right section with price
                                Col {
                                    class: "items-end",
                                    {price_display}
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}
