use crate::{
    components::*,
    config::Token,
    gateway::GatewayResult,
    hooks::{
        on_transaction_done, use_gateway, use_quote, use_swap_transaction,
        use_token_balance_for_token,
    },
};
use dioxus::prelude::*;
use jupiter_swap_api_client::quote::QuoteResponse;
use ore_types::request::TransactionType;
use rust_decimal::Decimal;
use solana_sdk::transaction::VersionedTransaction;

use super::TokenInputError;

#[component]
pub fn SwapForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();

    // Selected tokens
    let buy_token = use_signal(|| Some(Token::ore()));
    let sell_token = use_signal(|| Some(Token::sol()));

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

    // On successful transaction, reset input amounts
    on_transaction_done(move |_sig| {
        sell_input_amount.set("".to_string());
        buy_input_amount.set("".to_string());
        sell_token_balance.restart();
        buy_token_balance.restart();
    });

    rsx! {
        Col {
            class: "w-full gap-4 {class}",
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-lg z-0",
                TokenInputForm {
                    class: "p-4 border-b border-gray-800",
                    title: "Sell".to_string(),
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
                        title: "Buy".to_string(),
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
            SwapDetails {
                quote_response,
            }
            SwapButton {
                quote_response,
                swap_tx,
                err,
            }
        }
    }
}

#[component]
fn SwapDetails(quote_response: Signal<Option<QuoteResponse>>) -> Element {
    // let priority_fee = use_resource(move || async move {
    //     let gateway = use_gateway();
    //     // let priority_fee = gateway.get_recent_priority_fee_estimate(true).await;
    //     // priority_fee
    // });
    // log::info!("priority_fee: {:?}", priority_fee.cloned());
    // microlamports * price * 10^9

    let (price_impact_value, slippage, transaction_fee) = {
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
                let transaction_fee = "0.00005 SOL".to_string(); // TODO Get priority fee

                // Priority fee

                // Base fee

                // Ore fee

                (price_impact_value, slippage, transaction_fee)
            }
            None => ("–".to_string(), "–".to_string(), "–".to_string()),
        }
    };

    // let transaction_fee = "0.00005 SOL".to_string(); // TODO Get priority fee
    // let slippage = "0.00005 SOL".to_string(); // TODO Get priority fee

    rsx! {
            Col {
                class: "px-5",
                gap: 3,
                SwapDetailLabel { title: "Price impact", value: price_impact_value }
                FeeDetailValue {}
        }
    }
}

#[component]
pub fn FeeDetailValue(// base fee, priority fee, ore fee
) -> Element {
    let mut is_open = use_signal(|| false);
    let base_fee = 0.00001; // Example values - replace with actual fee calculations
    let priority_fee = 0.00002;
    let ore_fee = 0.00002;
    let total_fee = base_fee + priority_fee + ore_fee;

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
                        "Total Fee"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0",
                    }
                }
                Row {
                    class: "items-center gap-2",
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        { format!("{:.5} SOL", total_fee) }
                    }
                }
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
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{base_fee:.5} SOL" }
                        }
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "Solana base Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{priority_fee:.5} SOL" }
                        }
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "Solana priority Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{ore_fee:.5} SOL" }
                        }
                    }
                }
            }
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
fn SwapButton(
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
                        "Swap"
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
