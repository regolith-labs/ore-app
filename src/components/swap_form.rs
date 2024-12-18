use dioxus::prelude::*;
use dioxus_sdk::utils::timing::{use_debounce, UseDebounce};

use crate::{
    components::{Col, Row, SwitchIcon},
    hooks::{get_token_balance, use_wallet, Asset, GetPubkey, ASSETS},
};

#[component]
pub fn SwapForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    // inputs
    let mut sell_input_amount = use_signal::<String>(|| "".to_owned());
    let mut buy_input_amount = use_signal::<String>(|| "".to_owned());
    // enabled
    let mut enabled = use_signal(|| false);
    // show tokens
    let show_token_selector_a = use_signal(|| false);
    let show_token_selector_b = use_signal(|| false);
    // tokens
    let token_a = use_signal(|| Asset::ore());
    let token_b = use_signal(|| Asset::first());

    // quotes
    let sell_quote_debounce =
        use_debounce::<String>(std::time::Duration::from_secs(5), move |str| {
            let s = format!("{}9", str);
            spawn({
                async move {
                    sell_input_amount.set(s.clone());
                }
            });
        });
    let buy_quote_debounce =
        use_debounce::<String>(std::time::Duration::from_secs(5), move |str| {
            let s = format!("{}9", str);
            spawn({
                async move {
                    buy_input_amount.set(s.clone());
                }
            });
        });

    use_effect(move || {
        let amount_str = sell_input_amount.cloned();
        if amount_str.is_empty() {
            enabled.set(false);
            return;
        }

        let Ok(amount) = amount_str.parse::<f64>() else {
            enabled.set(false);
            return;
        };

        if amount == 0f64 {
            enabled.set(false);
            return;
        }

        enabled.set(true);
    });

    rsx! {
        Col { class: "w-full {class}", gap: 4,
            Col { class: "relative lg:flex elevated elevated-border shrink-0 h-min rounded-lg z-0",
                SwapInput {
                    mode: SwapInputMode::Buy,
                    input_amount: buy_input_amount,
                    show_selector: show_token_selector_a,
                    selected_token: token_a,
                    quote_debounce: buy_quote_debounce,
                }
                SwapInput {
                    mode: SwapInputMode::Sell,
                    input_amount: sell_input_amount,
                    show_selector: show_token_selector_b,
                    selected_token: token_b,
                    quote_debounce: sell_quote_debounce,
                }
                SwitchButton {
                    token_a,
                    token_b,
                    input_amount_a: buy_input_amount,
                    input_amount_b: sell_input_amount,
                }
            }
            SwapDetails { token_a, token_b }
            SwapButton { enabled }
            // Token selector popups
            if *show_token_selector_a.read() {
                TokenPicker {
                    show_token_selector: show_token_selector_a,
                    selected_token: token_a,
                    other_token: token_b,
                }
            }
            if *show_token_selector_b.read() {
                TokenPicker {
                    show_token_selector: show_token_selector_b,
                    selected_token: token_b,
                    other_token: token_a,
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
fn SwapDetails(token_a: Signal<Asset>, token_b: Signal<Asset>) -> Element {
    let (from_token, to_token) = (
        token_a.read().ticker.to_string(),
        token_b.read().ticker.to_string(),
    );

    rsx! {
        Col { class: "px-1", gap: 3,
            DetailLabel {
                title: "Price",
                value: format!("1 {from_token} = 0.5 {to_token}"),
            }
            DetailLabel { title: "Price impact", value: "0.5%" }
            DetailLabel { title: "Transaction fee", value: "0.00005 SOL" }
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
fn SwapButton(enabled: Signal<bool>) -> Element {
    let colors = if *enabled.read() {
        "controls-primary"
    } else {
        "bg-controls-disabled text-on-onDisabled"
    };
    rsx! {
        button {
            class: "h-12 w-full rounded-full {colors}",
            disabled: !*enabled.read(),
            onclick: move |_| {},
            span { class: "mx-auto my-auto font-semibold", "Swap" }
        }
    }
}

#[component]
fn SwitchButton(
    mut token_a: Signal<Asset>,
    mut token_b: Signal<Asset>,
    input_amount_a: Signal<String>,
    input_amount_b: Signal<String>,
) -> Element {
    rsx! {
        button {
            class: "absolute w-12 h-8 -mt-4 inset-y-1/2 -ml-4 inset-x-1/2 rounded elevated-control elevated-border text-elements-midEmphasis",
            onclick: move |_| {
                let token_a_peek = token_a.clone();
                let token_a_peek = token_a_peek.peek().clone();
                let token_b_peek = token_b.clone();
                let token_b_peek = token_b_peek.peek().clone();
                token_a.set(token_b_peek);
                token_b.set(token_a_peek);
                input_amount_a.set("0.0".to_string());
                input_amount_b.set("0.0".to_string());
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
    input_amount: Signal<String>,
    show_selector: Signal<bool>,
    selected_token: Signal<Asset>,
    quote_debounce: UseDebounce<String>,
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
    // let balance = use_token_balance(mint);
    let wallet = use_wallet();
    let balance = use_resource(move || async move {
        let wallet = wallet.get_pubkey()?;
        let asset = selected_token.read();
        get_token_balance(wallet, asset.mint).await
    });

    rsx! {
        Col { class: "w-full p-4 {border}", gap: 2,
            Row { class: "justify-between",
                span { class: "text-elements-midEmphasis my-auto pl-1", "{title}" }
                if let SwapInputMode::Sell = mode {
                    button {
                        class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                        onclick: move |_| {
                            if let Some(Ok(balance)) = balance.read().as_ref() {
                                log::info!("balance: ok {:?}", balance);
                                input_amount.set(balance.ui_amount.unwrap_or(0.0).to_string());
                            } else {
                                log::info!("balance: {:?}", balance);
                            }
                        },
                        "Max"
                    }
                }
            }
            Row { gap: 4,
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
                input {
                    class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                    placeholder: "0",
                    r#type: "number",
                    inputmode: "decimal",
                    value: input_amount.cloned(),
                    oninput: move |e| {
                        let s = e.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            log::info!("Ok... {s}");
                            quote_debounce.action(s);
                        } else {
                            let x = s[..s.len() - 1].to_string();
                            log::info!("Not ok... {s} yep {x}");
                            let s = s[..s.len() - 1].to_string();
                            quote_debounce.action(s);
                        }
                    },
                }
            }
        }
    }
}
