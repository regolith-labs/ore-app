use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{Col, Row, SwitchIcon};

#[component]
pub fn SwapForm(mint_a: Pubkey, mint_b: Pubkey) -> Element {
    let sell_input_amount = use_signal::<String>(|| "".to_owned());
    let buy_input_amount = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);

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
            gap: 4,
            Col {
                class: "relative lg:flex elevated elevated-border shrink-0 h-min w-96 rounded z-0",
                SwapInput {
                    mint: Pubkey::new_unique(),
                    mode: SwapInputMode::Sell,
                    input_amount: sell_input_amount,
                }
                SwapInput {
                    mint: Pubkey::new_unique(),
                    mode: SwapInputMode::Buy,
                    input_amount: buy_input_amount,
                }
                SwitchButton {}
            }
            SwapDetails {}
            SwapButton {
                enabled: enabled
            }
        }
    }
}

fn SwapDetails() -> Element {
    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Transaction fee",
                value: "0.00005 SOL"
            }
            DetailLabel {
                title: "Price impact",
                value: "0.5%"
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

fn SwitchButton() -> Element {
    rsx! {
        button {
            class: "absolute w-12 h-8 -mt-4 inset-y-1/2 -ml-4 inset-x-1/2 rounded elevated-control elevated-border text-elements-midEmphasis",
            onclick: move |_| {
                // TODO
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
fn SwapInput(mint: Pubkey, mode: SwapInputMode, input_amount: Signal<String>) -> Element {
    let border = match mode {
        SwapInputMode::Buy => "",
        SwapInputMode::Sell => "border-b border-gray-800",
    };
    let title = match mode {
        SwapInputMode::Buy => "Buying",
        SwapInputMode::Sell => "Selling",
    };

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
                if mode == SwapInputMode::Sell {
                    button {
                        class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                        onclick: move |_| {
                            // TODO
                        },
                        "Max"
                    }
                }
            }
            Row {
                gap: 4,
                Row {
                    class: "my-auto",
                    gap: 3,
                    img {
                        class: "w-8 h-8 rounded-full",
                        src: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png",
                    }
                    span {
                        class: "font-semibold my-auto",
                        "SOL"
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
