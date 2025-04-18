use dioxus::prelude::*;

use crate::components::*;
use crate::config::{Token, LISTED_TOKENS};
use crate::gateway::GatewayResult;
use crate::gateway::UiTokenAmount;
use crate::hooks::use_token_balance;
use crate::route::Route;

#[component]
pub(super) fn TokenTable(on_close: EventHandler<MouseEvent>) -> Element {
    let listed_tokens = LISTED_TOKENS.values().collect::<Vec<_>>();
    rsx! {
        Col {
            gap: 4,
            TableSimple {
                rows: rsx! {
                    for token in listed_tokens {
                        TokenRow { token: token.clone(), on_close: on_close }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenRow(token: Token, on_close: EventHandler<MouseEvent>) -> Element {
    let balance = use_token_balance(token.mint);
    rsx! {
        TableSimpleRowLink {
            to: Route::Trade {},
            left: rsx! { TokenNameAndBalance { token: token.clone(), balance } },
            right: rsx! { TokenQuote { token } },
            onclick: move |e| {
                on_close.call(e);
            }
        }
    }
}

#[component]
fn TokenNameAndBalance(token: Token, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
    rsx! {
        Row {
            gap: 4,
            img {
                class: "w-8 h-8 my-auto bg-gray-900 rounded-full",
                src: "{token.image}"
            }
            Col {
                span {
                    class: "font-medium",
                    "{token.ticker}"
                }
                span {
                    class: "font-medium text-gray-700 h-5 text-sm",
                    match balance.cloned() {
                        None => rsx! {
                            div { class: "h-5 w-20 loading rounded"}
                        },
                        Some(balance) => {
                            match balance {
                                Err(_) => rsx!{ "0" },
                                Ok(b) => rsx!{ "{b.ui_amount_string}" },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenQuote(token: Token) -> Element {
    rsx! {
        Col {
            class: "text-right",
            OreValueSmallAbbreviated {
                ui_amount_string: "1.20245"
            }
            span {
                class: "font-medium text-green-500 text-sm",
                "0.2%"
            }
        }
    }
}

#[component]
fn TokenValue(token: Token, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
    let mut value = use_signal(|| "0.000".to_string());
    let price = 1.2; // TODO

    use_effect(move || {
        if let Some(balance) = balance.cloned() {
            match balance {
                Err(_) => {
                    value.set("0.000".to_string());
                }
                Ok(balance) => {
                    if let Some(ui_amount) = balance.ui_amount {
                        value.set(format!("{:.3}", ui_amount * price));
                    }
                }
            }
        }
    });

    rsx! {
        if let Some(_balance) = balance.cloned() {
            OreValueSmallAbbreviated {
                ui_amount_string: "{*value.read()}"
            }
        } else {
            div {
                class: "loading w-24 h-8 rounded",
            }
        }
    }
}

#[component]
pub(super) fn LiquidityTable(on_close: EventHandler<MouseEvent>) -> Element {
    let listed_tokens = LISTED_TOKENS.values().collect::<Vec<_>>();
    rsx! {
        TableSimple {
            rows: rsx! {
                for token in listed_tokens {
                    LiquidityRow {
                        token: token.clone(),
                        on_close: on_close
                    }
                }
            }
        }
    }
}

#[component]
fn LiquidityRow(token: Token, on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        TableSimpleRowLink {
            // to: Route::Pair {
            //     pair: format!("{}-ORE", asset.ticker.clone())
            // },
            to: Route::Stake {},
            onclick: move |e| {
                on_close.call(e);
            },
            left: rsx! {
                Row {
                    class: "grow shrink-0",
                    gap: 4,
                    Row {
                        class: "shrink-0",
                        img {
                            class: "w-8 h-8 shrink-0 my-auto rounded-full",
                            src: "{token.image}"
                        }
                        img {
                            class: "w-8 h-8 shrink-0 -ml-2 my-auto rounded-full",
                            src: asset!("/public/icon.png")
                        }
                    }
                    Col {
                        class: "my-auto min-w-32 shrink-0",
                        span {
                            class: "font-medium whitespace-nowrap",
                            "{token.ticker}-ORE"
                        }
                        span {
                            class: "font-medium text-gray-700 h-5 text-sm",
                            "0"
                        }
                    }
                }
            },
            right: rsx! {
                Col {
                    class: "text-right",
                    OreValueSmallAbbreviated {
                        ui_amount_string: "2.054"
                    }
                    span {
                        class: "font-medium text-elements-gold text-sm",
                        "+0.123"
                    }
                }
            }
        }
    }
}
