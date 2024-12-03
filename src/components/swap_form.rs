use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{components::{Col, Row, SwitchIcon}, hooks::{use_token_balance, ASSETS}};

#[component]
pub fn SwapForm(
    class: Option<String>,
    mint_a: Pubkey,
    mint_b: Pubkey
) -> Element {
    let class = class.unwrap_or_default();
    let sell_input_amount = use_signal::<String>(|| "".to_owned());
    let buy_input_amount = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);
    let show_token_selector_a = use_signal(|| false);
    let show_token_selector_b = use_signal(|| false);
    let token_a = use_signal(|| "ORE".to_string());
    let token_b = use_signal(|| "SOL".to_string());
    let tokens_swapped = use_signal(|| false);

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
        Col {
            class: "w-full {class}",
            gap: 4,
            Col {
                class: "relative lg:flex elevated elevated-border shrink-0 h-min rounded z-0",
                SwapInput {
                    mint: Pubkey::new_unique(),
                    mode: SwapInputMode::Buy,
                    input_amount: buy_input_amount,
                    show_selector: show_token_selector_a,
                    selected_token: token_a,
                }
                SwapInput {
                    mint: Pubkey::new_unique(),
                    mode: SwapInputMode::Sell,
                    input_amount: sell_input_amount,
                    show_selector: show_token_selector_b,
                    selected_token: token_b,
                }
                SwitchButton {
                    tokens_swapped: tokens_swapped,
                    token_a: token_a,
                    token_b: token_b
                }
            }
            SwapDetails {
                token_a: token_a,
                token_b: token_b,
                tokens_swapped: tokens_swapped
            }
            SwapButton {
                enabled: enabled
            }
            // Token selector popups
            if *show_token_selector_a.read() {
                TokenPicker {
                    show_token_selector: show_token_selector_a,
                    selected_token: token_a,
                    other_token: token_b
                }
            }
            if *show_token_selector_b.read() {
                TokenPicker {
                    show_token_selector: show_token_selector_b, 
                    selected_token: token_b,
                    other_token: token_a
                }
            }
        }
    }
}

#[component]
fn TokenPicker(show_token_selector: Signal<bool>, selected_token: Signal<String>, other_token: Signal<String>) -> Element {
    let assets = ASSETS.values().collect::<Vec<_>>();
    let mut search = use_signal(|| String::new());
    let search_str = search.cloned();
    let other = other_token.read().to_string();
    let filtered_assets = assets.iter()
        .map(|a| (*a).clone())
        .filter(move |asset| {
            if search_str.is_empty() {
                asset.ticker != other
            } else {
                asset.ticker != other && asset.ticker.to_lowercase().contains(&search_str.to_lowercase())
            }
        }).collect::<Vec<_>>();

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
                        class: "w-full p-2 rounded bg-surface-secondary text-black",
                        placeholder: "Search by token name...",
                        oninput: move |e| search.set(e.value().clone())
                    }
                    
                    // Token list
                    Col {
                        gap: 2,
                        for asset in filtered_assets {
                            button {
                                class: "flex items-center gap-2 p-2 hover:bg-controls-secondaryHover rounded transition-colors duration-200",
                                onclick: move |_| {
                                    selected_token.set(asset.ticker.clone());
                                    show_token_selector.set(false);
                                },
                                img {
                                    class: "w-8 h-8 rounded-full",
                                    src: asset.image.clone()
                                }
                                span {
                                    class: "font-semibold",
                                    "{asset.ticker}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SwapDetails(token_a: Signal<String>, token_b: Signal<String>, tokens_swapped: Signal<bool>) -> Element {
    let (from_token, to_token) = if tokens_swapped.cloned() {
        (token_b.read().to_string(), token_a.read().to_string())
    } else {
        (token_a.read().to_string(), token_b.read().to_string())
    };

    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Price",
                value: format!("1 {from_token} = 0.5 {to_token}")
            }
            DetailLabel {
                title: "Price impact",
                value: "0.5%"
            }
            DetailLabel {
                title: "Transaction fee", 
                value: "0.00005 SOL"
            }
        }
    }
}

#[component]
fn DetailLabel(title: String, value: String) -> Element {
    rsx! {
        Row {
            class: "w-full justify-between text-sm",
            span {
                class: "text-elements-lowEmphasis",
                "{title}"
            }
            span {
                class: "text-elements-midEmphasis",
                "{value}"
            }
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
            class: "h-10 w-full rounded-full {colors}",
            disabled: !*enabled.read(),
            onclick: move |_| {
                // TODO
            },
            span {
                class: "mx-auto my-auto",
                "Swap"
            }
        }
    }
}

#[component]
fn SwitchButton(tokens_swapped: Signal<bool>, token_a: Signal<String>, token_b: Signal<String>) -> Element {
    rsx! {
        button {
            class: "absolute w-12 h-8 -mt-4 inset-y-1/2 -ml-4 inset-x-1/2 rounded elevated-control elevated-border text-elements-midEmphasis",
            onclick: move |_| {
                tokens_swapped.set(!tokens_swapped.cloned());
            },
            SwitchIcon {
                class: "h-4 mx-auto"
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum SwapInputMode {
    Buy,
    Sell,
}

#[component]
fn SwapInput(mint: Pubkey, mode: SwapInputMode, input_amount: Signal<String>, show_selector: Signal<bool>, selected_token: Signal<String>) -> Element {
    let border = match mode {
        SwapInputMode::Buy => "border-b border-gray-800",
        SwapInputMode::Sell => "",
    };
    let title = match mode {
        SwapInputMode::Buy => "Buy",
        SwapInputMode::Sell => "Sell",
    };

    let display_token = selected_token.read().to_string();
    let image = ASSETS.get(&display_token)
        .map(|asset| asset.image.clone())
        .unwrap_or_else(|| "icon.png".to_string());

    let balance = use_token_balance(mint);

    rsx! {
        Col {
            class: "w-full p-4 {border}",
            gap: 2,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-midEmphasis my-auto pl-1",
                    "{title}"
                }
                button {
                    class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                    onclick: move |_| {
                        if let SwapInputMode::Sell = mode {
                            if let Some(Ok(balance)) = balance.read().as_ref() {
                                input_amount.set(balance.ui_amount.unwrap_or(0.0).to_string());
                            }
                        }
                    },
                    "Max"
                }
            }
            Row {
                gap: 4,
                button {
                    class: "flex items-center gap-2 p-2 -ml-1 -mt-1 hover:bg-controls-secondaryHover rounded cursor-pointer shrink-0",
                    onclick: move |_| show_selector.set(true),
                    Row {
                        class: "my-auto",
                        gap: 2,
                        img {
                            class: "w-8 h-8 rounded-full shrink-0",
                            src: "{image}"
                        }
                        span {
                            class: "font-semibold my-auto",
                            "{display_token}"
                        }
                    }
                }
                input {
                    class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right",
                    placeholder: "0",
                    value: input_amount.cloned(),
                    oninput: move |e| {
                        let s = e.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            log::info!("Ok... {s}");
                            input_amount.set(s);
                        } else {
                            let x = s[..s.len()-1].to_string();
                            log::info!("Not ok... {s} yep {x}");
                            input_amount.set(s[..s.len()-1].to_string());
                        }
                    }
                }
            }
        }
    }
}
