use dioxus::prelude::*;
use dioxus_sdk::utils::timing::UseDebounce;
use jupiter_swap_api_client::quote::QuoteResponse;
use rust_decimal::Decimal;
use solana_client_wasm::solana_sdk::transaction::VersionedTransaction;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::{
    components::{invoke_signature, Col, InvokeSignatureStatus, Row, SwitchIcon},
    gateway::GatewayResult,
    hooks::{get_token_balance, use_quote, use_swap_transaction, use_wallet, Asset, GetPubkey, ASSETS},
};

#[component]
pub fn SwapForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();

    // wallet
    let wallet = use_wallet();

    // inputs
    let sell_input_amount = use_signal::<Option<String>>(|| Some("".to_owned()));
    let buy_input_amount = use_signal::<Option<String>>(|| Some("".to_owned()));

    // show tokens
    let show_buy_token_selector = use_signal(|| false);
    let show_sell_token_selector = use_signal(|| false);

    // tokens
    let buy_token = use_signal(|| Asset::ore());
    let sell_token = use_signal(|| Asset::first());

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

    // quote response
    let quote_response = use_signal::<Option<QuoteResponse>>(|| None);
    let mut invoke_signature_status = use_signal(|| InvokeSignatureStatus::Start);

    // reset signature
    use_effect(move || {
        let _ = buy_input_amount.read();
        let _ = sell_input_amount.read();
        invoke_signature_status.set(InvokeSignatureStatus::Start);
    });
    use_effect(move || {
        if let InvokeSignatureStatus::Done(_sig) = invoke_signature_status.cloned() {
            buy_token_balance.restart();
            sell_token_balance.restart();
            invoke_signature_status.set(InvokeSignatureStatus::Start);
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
        Col { class: "w-full {class}", gap: 4,
            Col { class: "relative lg:flex elevated elevated-border shrink-0 h-min rounded-lg z-0",
                SwapInput {
                    mode: SwapInputMode::Buy,
                    input_amount: buy_input_amount,
                    output_amount: sell_input_amount,
                    show_selector: show_buy_token_selector,
                    selected_token: buy_token,
                    selected_token_balance: buy_token_balance,
                    new_quote: buy_quote,
                }
                SwapInput {
                    mode: SwapInputMode::Sell,
                    input_amount: sell_input_amount,
                    output_amount: buy_input_amount,
                    show_selector: show_sell_token_selector,
                    selected_token: sell_token,
                    selected_token_balance: sell_token_balance,
                    new_quote: sell_quote,
                }
                SwitchButton {
                    buy_token,
                    sell_token,
                    buy_input_amount,
                    sell_input_amount,
                    quote_response
                }
            }
            SwapDetails { buy_token, sell_token, quote_response }
            SwapButton { quote_response, swap_tx, invoke_signature_status }

            div { "{invoke_signature_status}" }

            // Token selector popups
            if *show_buy_token_selector.read() {
                TokenPicker {
                    show_token_selector: show_buy_token_selector,
                    selected_token: buy_token,
                    other_token: sell_token,
                }
            }
            if *show_sell_token_selector.read() {
                TokenPicker {
                    show_token_selector: show_sell_token_selector,
                    selected_token: sell_token,
                    other_token: buy_token,
                }
            }
        }
    }
}

#[component]
fn TokenPicker(
    show_token_selector: Signal<bool>,
    selected_token: Signal<Asset>,
    other_token: Signal<Asset>,
) -> Element {
    let assets = ASSETS.values().collect::<Vec<_>>();
    let mut search = use_signal(|| String::new());
    let search_str = search.cloned();
    let selected = selected_token.read().ticker.to_string();
    let other = other_token.read().ticker.to_string();
    let filtered_assets = assets
        .iter()
        .map(|a| (*a).clone())
        .filter(move |asset| {
            if search_str.is_empty() {
                asset.ticker != other && asset.ticker != selected
            } else {
                asset.ticker != other
                    && asset.ticker != selected
                    && asset
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

                Col { gap: 4,
                    // Search input
                    input {
                        class: "w-full p-2 rounded bg-surface-secondary text-black",
                        placeholder: "Search by token name...",
                        oninput: move |e| search.set(e.value().clone()),
                    }

                    // Token list
                    Col { gap: 2,
                        for asset in filtered_assets {
                            button {
                                class: "flex items-center gap-2 p-2 hover:bg-controls-secondaryHover rounded transition-colors duration-200",
                                onclick: {
                                    let asset = asset.clone();
                                    move |_| {
                                        selected_token.set(asset.clone());
                                        show_token_selector.set(false);
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
    buy_token: Signal<Asset>,
    sell_token: Signal<Asset>,
    quote_response: Signal<Option<QuoteResponse>>,
) -> Element {
    let (price_value, price_impact_value, slippage) = {
        let quote_response = &*quote_response.read();
        match quote_response {
            Some(quote_response) => {
                // price value
                let sell_token = sell_token.read();
                let buy_token = buy_token.read();
                let price_ratio_numerator = (quote_response.out_amount as f64)
                    / (10u64.pow(buy_token.decimals as u32) as f64);
                let price_ratio_denomintaor = (quote_response.in_amount as f64)
                    / (10u64.pow(sell_token.decimals as u32) as f64);
                let price_ratio = price_ratio_numerator / price_ratio_denomintaor;
                let price_value = format!(
                    "1 {} = {:.2} {}",
                    sell_token.ticker, price_ratio, buy_token.ticker
                );

                // price impact value
                let price_impact_value = format!(
                    "{:.2}%",
                    quote_response
                        .price_impact_pct
                        .saturating_mul(Decimal::new(100, 0))
                );

                // slippage
                let slippage = format!("{:.2}%", (quote_response.slippage_bps as f64) / 1000f64);
                (price_value, price_impact_value, slippage)
            }
            None => ("".to_string(), "".to_string(), "".to_string()),
        }
    };

    rsx! {
        Col { class: "px-1", gap: 3,
            DetailLabel {
                title: "Price",
                value: price_value,
            }
            DetailLabel { title: "Price impact", value: price_impact_value }
            DetailLabel { title: "Slippage", value: slippage }
        }
    }
}

#[component]
fn DetailLabel(title: String, value: String) -> Element {
    rsx! {
        Row { class: "w-full justify-between text-sm",
            span { class: "text-elements-lowEmphasis", "{title}" }
            span { class: "text-elements-midEmphasis", "{value}" }
        }
    }
}

#[component]
fn SwapButton(
    quote_response: Signal<Option<QuoteResponse>>,
    swap_tx: Resource<GatewayResult<VersionedTransaction>>,
    invoke_signature_status: Signal<InvokeSignatureStatus>,
) -> Element {
    let quote_response = &*quote_response.read();
    let colors = if (quote_response).is_some() {
        "controls-primary"
    } else {
        "bg-controls-disabled text-on-onDisabled"
    };
    rsx! {
        button {
            class: "h-12 w-full rounded-full {colors}",
            disabled: quote_response.is_none() && swap_tx().is_some_and(|res| res.is_ok()),
            onclick: move |_| {
                let swap_tx = &*swap_tx.read();
                if let Some(Ok(tx)) = swap_tx {
                    invoke_signature(tx.clone(), invoke_signature_status);
                }
            },
            span { class: "mx-auto my-auto font-semibold", "Swap" }
        }
    }
}

#[component]
fn SwitchButton(
    mut buy_token: Signal<Asset>,
    mut sell_token: Signal<Asset>,
    buy_input_amount: Signal<Option<String>>,
    sell_input_amount: Signal<Option<String>>,
    mut quote_response: Signal<Option<QuoteResponse>>,
) -> Element {
    rsx! {
        button {
            class: "absolute w-12 h-8 -mt-4 inset-y-1/2 -ml-4 inset-x-1/2 rounded elevated-control elevated-border text-elements-midEmphasis",
            onclick: move |_| {
                // Swap tokens
                let buy_token_peek = buy_token.peek().clone();
                let sell_token_peek = sell_token.peek().clone();
                buy_token.set(sell_token_peek);
                sell_token.set(buy_token_peek);
                
                // Swap input amounts
                let buy_input_peek = buy_input_amount.peek().clone();
                let sell_input_peek = sell_input_amount.peek().clone();
                buy_input_amount.set(sell_input_peek);
                sell_input_amount.set(buy_input_peek);

                // Clear quote response
                quote_response.set(None);
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

#[component]
fn SwapInput(
    mode: SwapInputMode,
    input_amount: Signal<Option<String>>,
    output_amount: Signal<Option<String>>,
    show_selector: Signal<bool>,
    selected_token: Signal<Asset>,
    selected_token_balance: Resource<GatewayResult<UiTokenAmount>>,
    new_quote: UseDebounce<String>,
) -> Element {
    let border = match mode {
        SwapInputMode::Buy => "border-b border-gray-800",
        SwapInputMode::Sell => "",
    };
    let title = match mode {
        SwapInputMode::Buy => "Buy",
        SwapInputMode::Sell => "Sell",
    };
    let display_token = selected_token.read().ticker.to_string();
    let image = ASSETS.get(&display_token).map(|asset| asset.image.clone());

    rsx! {
        Col { class: "w-full p-4 {border}", gap: 2,
            Row { class: "justify-between",
                span { class: "text-elements-midEmphasis my-auto pl-1", "{title}" }
                if let SwapInputMode::Sell = mode {
                    button {
                        class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                        onclick: move |_| {
                            if let Some(Ok(balance)) = selected_token_balance.read().as_ref() {
                                new_quote.action(balance.ui_amount.unwrap_or(0.0).to_string());
                            }
                        },
                        "Max"
                    }
                }
            }
            Row { 
                class: "justify-between",
                gap: 4,
                button {
                    class: "flex items-center gap-2 p-2 -ml-1 -mt-1 hover:bg-controls-secondaryHover rounded cursor-pointer shrink-0",
                    onclick: move |_| {
                        if display_token.ne(&Asset::ore_ticker()) {
                            show_selector.set(true)
                        }
                    },
                    Row { class: "my-auto", gap: 2,
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
                        span { class: "font-semibold my-auto", "{display_token}" }
                    }
                }
                if let Some(input_amount) = input_amount.cloned() {
                    input {
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: input_amount,
                        oninput: move |e| {
                            new_quote.action(e.value());
                            output_amount.set(None);
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
