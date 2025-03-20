use dioxus::prelude::*;

use crate::{components::*, hooks::use_tokens_with_values, route::Route};

pub fn TokenList() -> Element {
    let tokens = use_tokens_with_values();

    rsx! {
        Col {
            class: "w-full",
            {tokens.iter().map(|token| {
                rsx! {
                    Link {
                        to: Route::TransferWithToken { token_ticker: token.token.ticker.clone() },
                        Row {
                            key: "{token.token.mint}",
                            class: "w-full justify-between items-center py-4 px-4 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
                            Row {
                                class: "items-center",
                                gap: 4,
                                img { class: "w-8 h-8 rounded-full shrink-0", src: "{token.token.image}" }
                                Col {
                                    span { class: "font-medium text-elements-highEmphasis", "{token.token.name}" }
                                    span { class: "font-medium text-xs text-elements-lowEmphasis",
                                        "{token.balance:.4} {token.token.ticker}"
                                    }
                                }
                            }
                            Col {
                                class: "items-end",
                                "${token.total_value:.2}"
                            }
                        }
                    }
                }
            })}
        }
    }
}
