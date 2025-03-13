use dioxus::prelude::*;

use crate::{
    components::*,
    config::{Token, LISTED_TOKENS},
};

// TODO Close on ESC click
#[component]
pub fn TokenPicker(
    show: Signal<bool>,
    token: Signal<Option<Token>>,
    on_tokens_change: Option<EventHandler<(Option<Token>, Option<Token>)>>,
    other_token: Option<Signal<Option<Token>>>,
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
                                        let old_token = token.cloned();

                                        // Select the new token
                                        token.set(Some(t.clone()));
                                        show.set(false);

                                        // Update URL if on_tokens_change is provided
                                        if let Some(on_tokens_change) = on_tokens_change {
                                            if let Some(other_token) = other_token {
                                                let other = other_token.cloned();
                                                on_tokens_change.call((other, Some(t.clone())));
                                            } else {
                                                on_tokens_change.call((old_token, Some(t.clone())));
                                            }
                                        }
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
