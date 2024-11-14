use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::SwitchIcon;

#[component]
pub fn SwapTool(mint_a: Pubkey, mint_b: Pubkey) -> Element {
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
        div {
            class: "flex flex-col gap-4",
            div {
                class: "relative lg:flex flex-col elevated elevated-border shrink-0 h-min w-96 rounded",
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
        div {
            class: "flex flex-col gap-3 px-1",
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
        div {
            class: "flex flex-row w-full justify-between text-sm",
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
            class: "absolute w-12 h-8 -mt-4 inset-y-1/2 -ml-4 inset-x-1/2 rounded elevated-control elevated-border text-elements-lowEmphasis",
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
        div {
            class: "flex flex-col gap-2 w-full p-4 {border}",
            div {
                class: "flex flex-row justify-between",
                span {
                    class: "text-gray-700 my-auto pl-1",
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
            div {
                class: "flex flex-row gap-4",
                div {
                    class: "flex flex-row gap-3 my-auto",
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
                    oninput: move |e| {
                        let s = e.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            input_amount.set(s);
                        } else {
                            input_amount.set(s[..s.len()-1].to_string());
                        }
                    }
                }
            }
        }
    }
}
