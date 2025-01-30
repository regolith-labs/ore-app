use dioxus::prelude::*;
use dioxus_sdk::utils::timing::UseDebounce;
use jupiter_swap_api_client::quote::QuoteResponse;
use rust_decimal::Decimal;
use solana_sdk::transaction::VersionedTransaction;

use crate::{
    components::{
        submit_transaction, CarrotDownIcon, Col, Row, SwitchIcon, TransactionStatus, WalletIcon
    },
    config::{Token, LISTED_TOKENS_BY_TICKER},
    gateway::{UiTokenAmount, GatewayResult},
    hooks::{
        get_token_balance, use_quote, use_swap_transaction, use_transaction_status, use_wallet, GetPubkey
    },
};

#[derive(Clone, PartialEq, Eq)]
pub enum SwapError {
    InsufficientBalance(String),
}

impl ToString for SwapError {
    fn to_string(&self) -> String {
        match self {
            SwapError::InsufficientBalance(ticker) => format!("Not enough {}", ticker),
        }
    }
}


#[component]
pub fn SwapForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    let wallet = use_wallet();

    // input amounts
    let sell_input_amount = use_signal::<Option<String>>(|| Some("".to_owned()));
    let buy_input_amount = use_signal::<Option<String>>(|| Some("".to_owned()));
    let transaction_status = use_transaction_status();

    // token picker modal flags
    let show_buy_token_selector = use_signal(|| false);
    let show_sell_token_selector = use_signal(|| false);

    // quote response
    let quote_response = use_signal::<Option<QuoteResponse>>(|| None);

    // selected tokens
    let buy_token = use_signal(|| Token::ore());
    let sell_token = use_signal(|| Token::sol());

    // Errors
    let mut error_msg = use_signal(|| None);

    // token balances
    let mut buy_token_balance = use_resource(move || async move {
        let wallet = wallet.get_pubkey()?;
        let buy_token = buy_token.read();
        get_token_balance(wallet, buy_token.mint).await
    });
    let mut sell_token_balance = use_resource(move || async move {
        let wallet = wallet.get_pubkey()?;
        let sell_token = sell_token.read();
        get_token_balance(wallet, sell_token.mint).await
    });

    // reset signature
    use_effect(move || {
        let _ = buy_input_amount.read();
        let _ = sell_input_amount.read();
    });
    use_effect(move || {
        if let Some(TransactionStatus::Done(_sig)) = transaction_status.cloned() {
            buy_token_balance.restart();
            sell_token_balance.restart();
        }
    });

    // quotes
    let buy_quote = use_quote(
        buy_token,
        buy_input_amount,
        sell_token,
        sell_input_amount,
        quote_response,
    );
    let sell_quote = use_quote(
        sell_token,
        sell_input_amount,
        buy_token,
        buy_input_amount,
        quote_response,
    );

    // swap
    let swap_tx = use_swap_transaction(quote_response);

    rsx! {
        Col {
            class: "w-full gap-4 {class}",
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-lg z-0",
                SwapInput {
                    mode: SwapInputMode::Sell,
                    input_amount: sell_input_amount,
                    other_amount: buy_input_amount,
                    show_selector: show_sell_token_selector,
                    selected_token: sell_token,
                    selected_token_balance: sell_token_balance,
                    new_quote: sell_quote,
                    quote_response,
                    error_msg,
                }
                div {
                    class: "relative",
                    SwapInput {
                        mode: SwapInputMode::Buy,
                        input_amount: buy_input_amount,
                        other_amount: sell_input_amount,
                        show_selector: show_buy_token_selector,
                        selected_token: buy_token,
                        selected_token_balance: buy_token_balance,
                        new_quote: buy_quote,
                        quote_response,
                        error_msg,
                    }
                    SwitchButton {
                        buy_token,
                        sell_token,
                        buy_input_amount,
                        sell_input_amount,
                        new_quote: sell_quote,
                        quote_response
                    }
                }
            }
            SwapDetails { 
                buy_token, 
                sell_token, 
                quote_response,
            }
            SwapButton { 
                quote_response, 
                swap_tx,
                error_msg,
            }
            if let Some(error_msg) = error_msg.cloned() {
                span { 
                    class: "text-red-500 font-semibold text-sm mx-auto my-auto", 
                    "{error_msg.to_string()}" 
                }
            }

            // TODO Signature status as toasts

            // Token selector popups
            if *show_buy_token_selector.read() {
                TokenPicker {
                    show_token_selector: show_buy_token_selector,
                    selected_token: buy_token,
                    other_token: sell_token,
                    buy_input_amount,
                    sell_input_amount,
                    sell_quote,
                    quote_response,
                }
            }
            if *show_sell_token_selector.read() {
                TokenPicker {
                    show_token_selector: show_sell_token_selector,
                    selected_token: sell_token,
                    other_token: buy_token,
                    buy_input_amount,
                    sell_input_amount,
                    sell_quote,
                    quote_response,
                }
            }
        }
    }
}

// TODO Close on ESC click
#[component]
fn TokenPicker(
    show_token_selector: Signal<bool>,
    selected_token: Signal<Token>,
    other_token: Signal<Token>,
    buy_input_amount: Signal<Option<String>>,
    sell_input_amount: Signal<Option<String>>,
    sell_quote: UseDebounce<String>,
    quote_response: Signal<Option<QuoteResponse>>,
) -> Element {
    let tokens = LISTED_TOKENS_BY_TICKER.values().collect::<Vec<_>>();
    let mut search = use_signal(|| String::new());
    let search_str = search.cloned();
    let selected = selected_token.read().ticker.to_string();
    let other = other_token.read().ticker.to_string();
    let filtered_assets = tokens
        .iter()
        .map(|t| (*t).clone())
        .filter(move |token| {
            if search_str.is_empty() {
                token.ticker != other && token.ticker != selected
            } else {
                token.ticker != other
                    && token.ticker != selected
                    && token
                        .ticker
                        .to_lowercase()
                        .contains(&search_str.to_lowercase())
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center",
            onclick: move |_| show_token_selector.set(false),
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
                        for asset in filtered_assets {
                            button {
                                class: "flex items-center gap-2 p-2 hover:bg-controls-secondaryHover rounded transition-colors duration-200",
                                onclick: {
                                    let asset = asset.clone();
                                    move |_| {
                                        // Select the new token
                                        selected_token.set(asset.clone());
                                        show_token_selector.set(false);

                                        // Get a new quote
                                        buy_input_amount.set(None);
                                        quote_response.set(None);
                                        sell_quote.action(sell_input_amount.cloned().unwrap_or_default());
                                    }
                                },
                                img {
                                    class: "w-8 h-8 rounded-full",
                                    src: asset.image,
                                }
                                span { class: "font-semibold", "{asset.ticker}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SwapDetails(
    buy_token: Signal<Token>,
    sell_token: Signal<Token>,
    quote_response: Signal<Option<QuoteResponse>>,
) -> Element {
    let (price_impact_value, _slippage, transaction_fee) = {
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
            // SwapDetailLabel { title: "Slippage", value: slippage }
            // SwapDetailLabel { title: "Transaction fee", value: transaction_fee }
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
    error_msg: Signal<Option<SwapError>>,
) -> Element {
    let quote_response = &*quote_response.read();
    let is_tx_ready = if let Some(Ok(_tx)) = swap_tx.cloned() {
        true
    } else {
        false
    };

    let is_disabled = quote_response.is_none() || !is_tx_ready || error_msg.read().is_some();

    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-primary transition-transform hover:not-disabled:scale-105",
            disabled: is_disabled,
            onclick: move |_| {
                let swap_tx = &*swap_tx.read();
                if let Some(Ok(tx)) = swap_tx {
                    submit_transaction(tx.clone());
                }
            },
            span { class: "mx-auto my-auto font-semibold", "Swap" }
        }
    }
}

#[component]
fn SwitchButton(
    mut buy_token: Signal<Token>,
    mut sell_token: Signal<Token>,
    buy_input_amount: Signal<Option<String>>,
    sell_input_amount: Signal<Option<String>>,
    mut quote_response: Signal<Option<QuoteResponse>>,
    mut new_quote: UseDebounce<String>,
) -> Element {
    rsx! {
        button {
            class: "absolute w-12 h-8 -mt-4 -ml-6 inset-y-0 inset-x-1/2 rounded elevated-control elevated-border text-elements-midEmphasis",
            onclick: move |_| {
                // Swap tokens
                let buy_token_peek = buy_token.peek().clone();
                let sell_token_peek = sell_token.peek().clone();
                buy_token.set(sell_token_peek);
                sell_token.set(buy_token_peek);

                // Swap input amounts
                let buy_input_peek = buy_input_amount.peek().clone();
                sell_input_amount.set(buy_input_peek.clone());

                // Get a new quote
                let mut needs_quote = false;
                if let Some(buy_input_peek) = buy_input_peek.clone() {
                    if !buy_input_peek.is_empty() {
                        buy_input_amount.set(None);
                        quote_response.set(None);
                        new_quote.action(buy_input_peek);
                        needs_quote = true;
                    }
                }
                if !needs_quote {
                    buy_input_amount.set(Some("".to_string()));
                }
            },
            SwitchIcon { class: "h-4 mx-auto" }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum SwapInputMode {
    Buy,
    Sell,
}

// TODO Amount too large error

#[component]
fn SwapInput(
    mode: SwapInputMode,
    input_amount: Signal<Option<String>>,
    other_amount: Signal<Option<String>>,
    show_selector: Signal<bool>,
    selected_token: Signal<Token>,
    selected_token_balance: Resource<GatewayResult<UiTokenAmount>>,
    new_quote: UseDebounce<String>,
    quote_response: Signal<Option<QuoteResponse>>,
    error_msg: Signal<Option<SwapError>>,
) -> Element {
    let border = match mode {
        SwapInputMode::Buy => "",
        SwapInputMode::Sell => "border-b border-gray-800",
    };
    let title = match mode {
        SwapInputMode::Buy => "Buy",
        SwapInputMode::Sell => "Sell",
    };

    // Set error message
    use_effect({
        let mode = mode.clone();
        move || {
            let mut is_error = false;
            if SwapInputMode::Sell == mode.clone() {
                if let Some(input_amount_str) = input_amount.cloned() {
                    let input_amount_f64 = input_amount_str.parse::<f64>().unwrap_or(0.0);
                    if let Some(Ok(balance)) = selected_token_balance.cloned() {
                        if balance.ui_amount.unwrap_or(0.0) < input_amount_f64 {
                            error_msg.set(Some(SwapError::InsufficientBalance(selected_token.read().ticker.to_string())));
                            is_error = true;
                        }
                    } else if input_amount_f64 > 0.0 {
                        error_msg.set(Some(SwapError::InsufficientBalance(selected_token.read().ticker.to_string())));
                        is_error = true;
                    }
                }
            }
            if !is_error {
                error_msg.set(None);
            }
        }
    });

    // Set input color
    let input_color = if let Some(error_msg) = error_msg.cloned() {
        match error_msg {
            SwapError::InsufficientBalance(ticker) => {
                if ticker == selected_token.read().ticker.to_string() {
                    "text-red-500"
                } else {
                    "text-elements-highEmphasis"
                }
            }
        }
    } else {
        "text-elements-highEmphasis"
    };

    rsx! {
        Col {
            class: "w-full p-4 {border}",
            gap: 2,
            Row {
                class: "justify-between",
                span { 
                    class: "text-elements-lowEmphasis my-auto pl-1", 
                    "{title}" 
                }
                Row {
                    gap: 2,
                    MaxButton { selected_token_balance, input_amount, other_amount, error_msg, quote_response, new_quote }
                }
            }
            Row {
                class: "justify-between",
                gap: 4,
                TokenButton { token: selected_token, show_selector: show_selector.clone() }
                if let Some(input_amount_str) = input_amount.cloned() {
                    input {
                        class: "{input_color} text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: input_amount_str,
                        oninput: move |e| {
                            // Nullify quote response
                            quote_response.set(None);

                            // Trigger updated quote
                            let s = e.value();
                            input_amount.set(Some(s.clone()));
                            new_quote.action(s.clone());

                            // Nullify other amount input field
                            if s.is_empty() {
                                other_amount.set(Some("".to_string()));
                            } else {
                                other_amount.set(None);
                            }
                        },
                    }
                } else {
                    span {
                        class: "h-10 w-32 loading rounded ml-auto"
                    }
                }
            }
        }
    }
}

#[component]
fn TokenButton(token: Signal<Token>, show_selector: Signal<bool>) -> Element {
    let ticker = token.read().ticker.to_string();
    let image = LISTED_TOKENS_BY_TICKER.get(&ticker).map(|token| token.image.clone());
    rsx! {
        button {
            class: "flex items-center gap-2 p-2 -ml-1 -mt-1 hover:bg-controls-secondaryHover rounded cursor-pointer shrink-0",
            onclick: move |_| show_selector.set(true),
            Row {
                class: "my-auto",
                gap: 2,
                if let Some(image) = image {
                    img {
                        class: "w-8 h-8 rounded-full shrink-0",
                        src: "{image}",
                    }
                } else {
                    img {
                        class: "w-8 h-8 rounded-full shrink-0",
                        src: asset!("/public/icon.png"),
                    }
                }
                span {
                    class: "font-semibold my-auto",
                    "{ticker}"
                }
                CarrotDownIcon {
                    class: "w-4 my-auto opacity-50" 
                }
            }
        }
    }
}

#[component]
fn MaxButton(
    selected_token_balance: Resource<GatewayResult<UiTokenAmount>>,
    input_amount: Signal<Option<String>>,
    other_amount: Signal<Option<String>>,
    error_msg: Signal<Option<SwapError>>,
    quote_response: Signal<Option<QuoteResponse>>,
    new_quote: UseDebounce<String>,
) -> Element {
    // Normalize token balance
    let token_balance = if let Some(Ok(balance)) = selected_token_balance.cloned() {
        balance.ui_amount.unwrap_or(0.0)
    } else {
        0.0
    };

    rsx! {
        button {
            class: "flex flex-row gap-2 py-1 px-1 font-medium text-elements-lowEmphasis hover:text-elements-highEmphasis hover:cursor-pointer my-auto",
            onclick: move |_| {
                if let Some(Ok(balance)) = selected_token_balance.read().as_ref() {
                    let max_amount = balance.ui_amount.unwrap_or(0.0);
                    if max_amount == 0.0 {
                        input_amount.set(Some("0".to_string()));
                        other_amount.set(Some("0".to_string()));
                        quote_response.set(None);
                        error_msg.set(None);
                    } else {
                        input_amount.set(Some(max_amount.to_string()));
                        other_amount.set(None);
                        quote_response.set(None);
                        new_quote.action(max_amount.to_string());
                        error_msg.set(None);
                    }
                } else {
                    input_amount.set(Some("0".to_string()));
                    other_amount.set(Some("0".to_string()));
                    quote_response.set(None);
                    error_msg.set(None);
                }
            },
            WalletIcon { class: "h-4 my-auto" }
            span { 
                class: "my-auto text-xs font-medium", 
                "{token_balance}" 
            }
        }
    }
}
