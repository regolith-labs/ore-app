use crate::hooks::{APP_FEE, SOLANA_BASE_FEE};
use crate::{
    components::*,
    config::Token,
    gateway::GatewayResult,
    hooks::{on_transaction_done, use_quote, use_swap_transaction, use_token_balance_for_token},
};
use dioxus::prelude::*;
use jupiter_swap_api_client::quote::QuoteResponse;
use ore_types::request::TransactionType;
use rust_decimal::Decimal;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::transaction::VersionedTransaction;

use super::TokenInputError;

/*
Require:
- min # of orders is 2
- min allocation for 2 order is $100

Order summary
- sell total
- sell per order
- to buy -> Ticker
- Order interval (frequency)
- Estimated end date
- Platform fee

First purchase is made immediately after user orders DCA +/- 30 seconds
We'll make ata if the user does not have one
All of the sell tokkens will be stored in user's DCA vault

inAta -> ata
outAta

*/

#[component]
pub fn RecurringForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();

    // Selected tokens
    let buy_token = use_signal(|| Some(Token::ore()));
    let sell_token = use_signal(|| Some(Token::usdc()));

    // Input amounts
    let mut sell_input_amount = use_signal::<String>(|| "".to_string());
    let mut buy_input_amount = use_signal::<String>(|| "".to_string());

    // Fetch token balances
    let mut sell_token_balance = use_token_balance_for_token(sell_token);
    let mut buy_token_balance = use_token_balance_for_token(buy_token);

    // Quote response
    let mut quote_response = use_signal::<Option<QuoteResponse>>(|| None);
    let mut err = use_signal::<Option<TokenInputError>>(|| None);

    // Quote fetcher with debounce
    let mut quote_fetcher = use_quote(sell_token, buy_token, buy_input_amount, quote_response);

    // Priority fee
    let mut priority_fee = use_signal::<u64>(|| 0);

    // When sell input amount changes, fetch a new quote
    use_effect(move || {
        let sell_input_amount = sell_input_amount.read().clone();
        let _sell_token = sell_token.read().clone();
        let _buy_token = buy_token.read().clone();
        err.set(None);
        buy_input_amount.set("".to_string());
        quote_response.set(None);
        quote_fetcher.action(sell_input_amount);
    });

    // Build swap transaction
    let swap_tx = use_swap_transaction(
        quote_response,
        sell_token,
        sell_token_balance,
        priority_fee,
        err,
    );

    priority_fee.set(0000000000);

    // On successful transaction, reset input amounts
    on_transaction_done(move |_sig| {
        sell_input_amount.set("".to_string());
        buy_input_amount.set("".to_string());
        sell_token_balance.restart();
        buy_token_balance.restart();
    });

    let placeholder = "1";
    let mut value = use_signal(|| "1".to_string());
    let mut update = use_signal(|| "1".to_string());

    // todo: make every component
    // todo: make over component
    rsx! {
        Col {
            class: "w-full gap-4 {class}",
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-lg z-0",
                TokenInputForm {
                    class: "p-4 border-b border-gray-800",
                    title: "Sell Recurring".to_string(),
                    token: sell_token,
                    balance: sell_token_balance,
                    value: sell_input_amount,
                    update: sell_input_amount,
                    toolbar_shortcuts: true,
                    with_picker: true,
                    err,
                }
                div {
                    class: "relative",
                    TokenInputForm {
                        class: "p-4",
                        title: "To Buy".to_string(),
                        token: buy_token,
                        balance: buy_token_balance,
                        value: buy_input_amount,
                        update: buy_input_amount,
                        loading: !sell_input_amount.read().is_empty() && quote_response.cloned().is_none(),
                        disabled: true,
                        with_picker: true,
                        err,
                    }
                    SwitchButton {
                        buy_token,
                        sell_token,
                        buy_input_amount,
                        sell_input_amount,
                    }
                }
            }
            Row {
                class: "w-full",
                gap: 4,
                Col {
                    class: "w-1/2",
                    Row {
                        class: "w-full elevated elevated-border shrink-0 h-min rounded-lg p-4",
                        Col {
                            Row {
                                class: "w-full text-elements-lowEmphasis my-auto",
                                "Every"
                            }
                            Row {
                                class: "justify-between",
                                input {
                                    class: "text-lg placeholder:text-gray-700 font-semibold bg-transparent h-12 pr-1 my-auto w-full outline-none text-left [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                    placeholder: placeholder.clone(),
                                    r#type: "number",
                                    step: "any",
                                    inputmode: "numeric",
                                    value: value.read().clone(),
                                    oninput: move |e: FormEvent| update.set(e.value()),
                                }
                                div {
                                    class: "relative w-full",
                                    select {
                                        class: "text-md placeholder:text-gray-700 bg-transparent h-12 pr-1 my-auto w-full outline-none text-right appearance-none",
                                        value: value.read().clone(),
                                        oninput: move |e: FormEvent| update.set(e.value()),
                                        option {
                                            value: "minute",
                                            selected: true,
                                            "Minute"
                                        }
                                        option {
                                            value: "hour",
                                            "Hour"
                                        }
                                        option {
                                            value: "day",
                                            "Day"
                                        }
                                        option {
                                            value: "week",
                                            "Week"
                                        }
                                        option {
                                            value: "month",
                                            "Month"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Col {
                    class: "w-1/2",
                    Row {
                        class: "w-full elevated elevated-border shrink-0 h-min rounded-lg p-4",
                        Col {
                            Row {
                                class: "w-full text-elements-lowEmphasis my-auto",
                                "Over"
                            }
                            Row {
                                class: "justify-between",
                                input {
                                    class: "text-lg placeholder:text-gray-700 font-semibold bg-transparent h-12 pr-1 my-auto w-full outline-none text-left [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                    placeholder: placeholder.clone(),
                                    r#type: "number",
                                    step: "any",
                                    inputmode: "numeric",
                                    value: value.read().clone(),
                                    oninput: move |e: FormEvent| update.set(e.value()),
                                }
                                span {
                                    class: "relative h-12 pr-1 my-auto flex items-center justify-end text-right",
                                    "orders"
                                }
                            }
                        }
                    }
                },

            }
            // SwapDetails {
            //     quote_response,
            //     priority_fee,
            // }
            OrderSummary { priority_fee: priority_fee.clone() }
            DCAButton {
                quote_response,
                swap_tx,
                err,
            }
        }
    }
}

fn format_fee(amount: f64) -> String {
    // Remove trailing zeros after decimal point
    let s = format!("{:.9}", amount);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[component]
pub fn OrderSummary(priority_fee: Signal<u64>) -> Element {
    let mut is_open = use_signal(|| false);
    let base_fee = lamports_to_sol(SOLANA_BASE_FEE);
    let app_fee = lamports_to_sol(APP_FEE);
    let priority_fee = lamports_to_sol(priority_fee.cloned());

    let total_fee = base_fee + priority_fee + app_fee;

    let max_height = if *is_open.read() {
        "max-h-32"
    } else {
        "max-h-0"
    };
    let opacity = if *is_open.read() {
        "opacity-100"
    } else {
        "opacity-0"
    };

    rsx! {
        button {
            class: "w-full flex flex-col transition-all duration-300 ease-in-out hover:cursor-pointer".to_string(),
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "w-full justify-between items-center",
                Row {
                    class: "text-elements-lowEmphasis gap-2 items-center",
                    span {
                        class: "font-medium",
                        "Order Summary"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0",
                    }
                }
                // Row {
                //     class: "items-center gap-2",
                //     span {
                //         class: "text-elements-lowEmphasis font-medium",
                //         { format!("{} SOL", format_fee(total_fee)) }
                //     }
                // }
            }
            Col {
                gap: 3,
                Col {
                    class: "overflow-hidden transition-all duration-300 ease-in-out {max_height}",
                    Col {
                        class: "pt-4 gap-2 transition-opacity duration-300 {opacity}",
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "App Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{format_fee(app_fee)}" }
                        }
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "Solana base Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{format_fee(base_fee)}" }
                        }
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "Solana priority Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{format_fee(priority_fee)}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SwapDetails(
    quote_response: Signal<Option<QuoteResponse>>,
    priority_fee: Signal<u64>,
) -> Element {
    let (price_impact_value, _slippage, _transaction_fee) = {
        let quote_response = &*quote_response.read();
        match quote_response {
            Some(quote_response) => {
                // price impact value
                let price_impact_value = format!(
                    "{:.2}%",
                    quote_response
                        .price_impact_pct
                        .saturating_mul(Decimal::new(100, 0))
                );

                // slippage
                let slippage = format!("{:.2}%", (quote_response.slippage_bps as f64) / 1000f64);

                // transaction fee
                let transaction_fee = "0.00005 SOL".to_string();

                (price_impact_value, slippage, transaction_fee)
            }
            None => ("–".to_string(), "–".to_string(), "–".to_string()),
        }
    };

    rsx! {
            Col {
                class: "px-5",
                gap: 3,
                SwapDetailLabel { title: "Price impact", value: price_impact_value }
                Fee { priority_fee: priority_fee.clone() }
        }
    }
}

#[component]
fn SwapDetailLabel(title: String, value: String) -> Element {
    rsx! {
        Row {
            class: "w-full justify-between",
            span { class: "text-elements-lowEmphasis font-medium", "{title}" }
            span { class: "text-elements-midEmphasis font-medium", "{value}" }
        }
    }
}

#[component]
fn DCAButton(
    quote_response: Signal<Option<QuoteResponse>>,
    swap_tx: Resource<GatewayResult<VersionedTransaction>>,
    err: Signal<Option<TokenInputError>>,
) -> Element {
    let quote_response = &*quote_response.read();
    let is_tx_ready = use_memo(move || {
        if let Some(Ok(_tx)) = swap_tx.cloned() {
            true
        } else {
            false
        }
    });

    let is_disabled = quote_response.is_none() || !is_tx_ready.cloned() || err.read().is_some();

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            button {
                class: "h-12 w-full rounded-full controls-primary transition-transform hover:not-disabled:scale-105",
                disabled: is_disabled,
                onclick: move |_| {
                    let swap_tx = &*swap_tx.read();
                    if let Some(Ok(tx)) = swap_tx {
                        submit_transaction(tx.clone(), TransactionType::Swap);
                    }
                },
                if let Some(err) = err.cloned() {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "{err.to_string()}"
                    }
                } else {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "Place DCA Order"
                    }
                }
            }
            Alert {}
        }
    }
}

#[component]
fn SwitchButton(
    buy_token: Signal<Option<Token>>,
    sell_token: Signal<Option<Token>>,
    buy_input_amount: Signal<String>,
    sell_input_amount: Signal<String>,
) -> Element {
    rsx! {
        button {
            class: "absolute flex w-12 h-12 -mt-6 -ml-6 inset-y-0 inset-x-1/2 rounded-full controls-tertiary hover:cursor-pointer",
            onclick: move |_| {
                // Swap tokens
                let buy_token_peek = buy_token.peek().clone();
                let sell_token_peek = sell_token.peek().clone();
                buy_token.set(sell_token_peek);
                sell_token.set(buy_token_peek);

                // Swap input amounts
                let buy_input_peek = buy_input_amount.peek().clone();
                sell_input_amount.set(buy_input_peek.clone());
                buy_input_amount.set("".to_string());
            },
            SwitchIcon {
                class: "h-4 mx-auto my-auto"
            }
        }
    }
}
