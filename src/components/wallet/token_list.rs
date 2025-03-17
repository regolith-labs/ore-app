use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

use crate::{
    components::*,
    config::{Token, LISTED_TOKENS},
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{use_ore_price, use_token_balance, use_wallet, OrePrice, Wallet},
};

// Token list component
pub fn TokenList() -> Element {
    let token_list: Vec<Token> = LISTED_TOKENS.values().cloned().collect();
    let ore_price = use_ore_price();

    // Get all token balances at top level
    let mut all_balances = HashMap::new();
    for token in token_list.iter() {
        let balance = use_token_balance(token.mint);
        all_balances.insert(token.mint, balance);
    }

    // Ore price at top level
    let ore_price_value = ore_price.read().clone();

    // Now use_memo with values we already have
    let tokens_with_data = use_memo(move || {
        token_list
            .iter()
            .filter_map(|token| {
                let balance = all_balances.get(&token.mint).unwrap();

                // Get the balance if available
                if let Some(Ok(amount)) = balance.read().as_ref() {
                    if let Some(ui_amount) = amount.ui_amount {
                        if ui_amount > 0.0 {
                            // Calculate price based on previously fetched ore_price
                            let price = if token.ticker == "ORE" {
                                ore_price_value.clone().map(|OrePrice(price)| price)
                            } else {
                                match token.ticker.as_str() {
                                    "SOL" => Some(21.30),
                                    "USDC" => Some(1.00),
                                    "HNT" => Some(3.47),
                                    _ => Some(0.0),
                                }
                            };

                            return Some((token.clone(), ui_amount, price));
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
                tokens_with_data.read().iter().map(|(token, ui_amount, price)| {
                    rsx! {
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
                                if let Some(price) = price {
                                    span {
                                        class: "font-medium text-elements-highEmphasis",
                                        "${price:.2}"
                                    }
                                } else {
                                    span {
                                        class: "font-medium text-elements-highEmphasis",
                                        "Price N/A"
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}
