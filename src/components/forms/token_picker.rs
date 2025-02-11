use dioxus::prelude::*;

use crate::{components::*, config::{Token, LISTED_TOKENS}};

// TODO Close on ESC click
#[component]
pub fn TokenPicker(
    show: Signal<bool>,
    token: Signal<Option<Token>>,
    // buy_input_amount: Signal<String>,
    // sell_input_amount: Signal<String>,
    // sell_quote: UseDebounce<String>,
    // quote_response: Signal<Option<QuoteResponse>>,
) -> Element {
    let tokens = LISTED_TOKENS.values().collect::<Vec<_>>();
    let mut search = use_signal(|| String::new());
    let search_str = search.cloned();
    let filtered_tokens = tokens
        .iter()
        .map(|t| (*t).clone())
        .filter(move |token| {
            if search_str.is_empty() {
                true
            } else {
                token
                    .ticker
                    .to_lowercase()
                    .contains(&search_str.to_lowercase())
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center",
            onclick: move |_| show.set(false),
            div {
                class: "bg-black rounded-lg p-4 w-96 border border-gray-800",
                onclick: move |e| e.stop_propagation(),
                Col {
                    gap: 4,

                    // Search input
                    input {
                        class: "w-full p-2 rounded bg-transparent text-elements-highEmphasis",
                        placeholder: "Search...",
                        oninput: move |e| search.set(e.value().clone()),
                    }

                    // Token list
                    Col {
                        gap: 2,
                        for t in filtered_tokens {
                            button {
                                class: "flex items-center gap-2 p-2 hover:bg-controls-secondaryHover rounded transition-colors duration-200 hover:cursor-pointer",
                                onclick: {
                                    let t = t.clone();
                                    move |_| {
                                        // Select the new token
                                        token.set(Some(t.clone()));
                                        show.set(false);

                                        // Get a new quote
                                        // buy_input_amount.set("".to_string());
                                        // quote_response.set(None);
                                        // sell_quote.action(sell_input_amount.cloned());
                                    }
                                },
                                img {
                                    class: "w-8 h-8 rounded-full",
                                    src: "{t.image}",
                                }
                                span { 
                                    class: "font-semibold",
                                    "{t.ticker}" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}