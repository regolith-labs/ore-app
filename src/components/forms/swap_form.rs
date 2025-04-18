use crate::{
    components::*,
    config::Token,
    gateway::{GatewayError, GatewayResult},
    hooks::{on_transaction_done, use_quote, use_swap_transaction, use_token_balance_wss},
};
use dioxus::prelude::*;
use jupiter_swap_api_client::quote::QuoteResponse;
use ore_types::request::TransactionType;
use rust_decimal::Decimal;
use solana_sdk::transaction::VersionedTransaction;

use super::TokenInputError;

#[component]
pub fn SwapForm(
    class: Option<String>,
    buy_token: Signal<Option<Token>>,
    sell_token: Signal<Option<Token>>,
    on_tokens_change: Option<EventHandler<(Option<Token>, Option<Token>)>>,
) -> Element {
    let class = class.unwrap_or_default();

    // Input amounts
    let mut sell_input_amount = use_signal::<String>(|| "".to_string());
    let mut buy_input_amount = use_signal::<String>(|| "".to_string());

    // Fetch token balances
    let mut sell_token_balance = use_signal(|| Err(GatewayError::AccountNotFound));
    use_effect(move || {
        let sell_token = sell_token.cloned();
        match sell_token {
            Some(token) => {
                let wss = use_token_balance_wss(&token.mint);
                let read = wss.cloned();
                sell_token_balance.set(read);
            }
            None => {
                sell_token_balance.set(Err(GatewayError::AccountNotFound));
            }
        }
    });
    let mut buy_token_balance = use_signal(|| Err(GatewayError::AccountNotFound));
    use_effect(move || {
        let buy_token = buy_token.cloned();
        match buy_token {
            Some(token) => {
                let wss = use_token_balance_wss(&token.mint);
                let read = wss.cloned();
                buy_token_balance.set(read);
            }
            None => {
                buy_token_balance.set(Err(GatewayError::AccountNotFound));
            }
        }
    });

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

    // TODO: remove priority fee from use_effect once we finish swap priority fee. Including now to avoid rerenders on write
    use_effect(move || {
        priority_fee.set(0000000000);
    });

    // On successful transaction, reset input amounts
    on_transaction_done(move |_sig| {
        sell_input_amount.set("".to_string());
        buy_input_amount.set("".to_string());
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
                    on_tokens_change: on_tokens_change.clone(),
                    other_token: Some(buy_token),
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
                        on_tokens_change,
                        other_token: Some(sell_token),
                    }
                    SwitchButton {
                        buy_token,
                        sell_token,
                        buy_input_amount,
                        sell_input_amount,
                        on_tokens_change,
                    }
                }
            }
            SwapDetails {
                quote_response,
                priority_fee,
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
            class: "px-4",
            gap: 2,
            SwapDetailLabel { title: "Price impact", value: price_impact_value }
            Fee {}
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
                class: "h-12 w-full rounded-full controls-primary transition-all duration-300 ease-in-out hover:not-disabled:scale-105",
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
    on_tokens_change: Option<EventHandler<(Option<Token>, Option<Token>)>>,
) -> Element {
    rsx! {
        button {
            class: "absolute flex w-12 h-12 -mt-6 -ml-6 inset-y-0 inset-x-1/2 rounded-full controls-tertiary hover:cursor-pointer",
            onclick: move |_| {
                // Swap tokens
                let buy_token_peek = buy_token.read().clone();
                let sell_token_peek = sell_token.read().clone();
                buy_token.set(sell_token_peek.clone());
                sell_token.set(buy_token_peek.clone());

                // Swap input amounts
                let buy_input_peek = buy_input_amount.peek().clone();
                sell_input_amount.set(buy_input_peek.clone());
                buy_input_amount.set("".to_string());

                // Call on_tokens_change event
                if let Some(on_tokens_change) = on_tokens_change {
                    on_tokens_change.call((sell_token_peek, buy_token_peek));
                }
            },
            SwitchIcon {
                class: "h-4 mx-auto my-auto"
            }
        }
    }
}
