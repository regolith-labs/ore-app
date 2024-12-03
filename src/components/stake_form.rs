use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{Col, Row};

#[component]
pub fn StakeForm(
    class: Option<String>,
    mint: Pubkey
) -> Element {
    let class = class.unwrap_or_default();
    let stake_amount_a = use_signal::<String>(|| "".to_owned());
    let stake_amount_b = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);
    let mut single_asset = use_signal(|| true);

    use_effect(move || {
        let amount_str_a = stake_amount_a.cloned();
        let amount_str_b = stake_amount_b.cloned();

        if *single_asset.read() {
            if amount_str_a.is_empty() {
                enabled.set(false);
                return;
            }

            let Ok(amount) = amount_str_a.parse::<f64>() else {
                enabled.set(false);
                return;
            };

            if amount == 0f64 {
                enabled.set(false);
                return;
            }
        } else {
            if amount_str_a.is_empty() || amount_str_b.is_empty() {
                enabled.set(false);
                return;
            }

            let Ok(amount_a) = amount_str_a.parse::<f64>() else {
                enabled.set(false);
                return;
            };

            let Ok(amount_b) = amount_str_b.parse::<f64>() else {
                enabled.set(false);
                return;
            };

            if amount_a == 0f64 || amount_b == 0f64 {
                enabled.set(false);
                return;
            }
        }

        enabled.set(true);
    });

    rsx! {
        Col {
            class: "w-full {class}",
            gap: 4,
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded z-0",
                StakeInputs {
                    mint: mint,
                    amount_a: stake_amount_a,
                    amount_b: stake_amount_b,
                    single_asset: single_asset
                }
            }
            Row {
                class: "px-1",
                label {
                    class: "flex items-center gap-2 text-sm text-elements-lowEmphasis cursor-pointer",
                    input {
                        r#type: "checkbox",
                        checked: *single_asset.read(),
                        oninput: move |_| {
                            single_asset.set(!single_asset.cloned())
                        }
                    }
                    "Single asset deposit"
                }
            }
            StakeDetails {}
            StakeButton {
                enabled: enabled
            }
        }
    }
}

fn StakeDetails() -> Element {
    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Transaction fee",
                value: "0.00005 SOL"
            }
            DetailLabel {
                title: "Yield",
                value: "1 ORE / day"
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
fn StakeButton(enabled: Signal<bool>) -> Element {
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
                // TODO: Implement staking logic
            },
            span {
                class: "mx-auto my-auto",
                "Stake"
            }
        }
    }
}

#[component]
fn StakeInputs(mint: Pubkey, amount_a: Signal<String>, amount_b: Signal<String>, single_asset: Signal<bool>) -> Element {
    rsx! {
        Col {
            class: "w-full p-4",
            gap: 2,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-midEmphasis my-auto pl-1",
                    "Deposit"
                }
                button {
                    class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                    onclick: move |_| {
                        // TODO: Implement max amount logic
                    },
                    "Max"
                }
            }
            Col {
                gap: if *single_asset.read() { 0 } else { 4 },
                Row {
                    gap: 4,
                    Row {
                        class: "my-auto",
                        gap: 2,
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
                        value: amount_a.cloned(),
                        oninput: move |e| {
                            let s = e.value();
                            if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                amount_a.set(s);
                            } else {
                                amount_a.set(s[..s.len()-1].to_string());
                            }
                        }
                    }
                }
                if !*single_asset.read() {
                    Row {
                        gap: 4,
                        Row {
                            class: "my-auto",
                            gap: 2,
                            img {
                                class: "w-8 h-8 rounded-full",
                                src: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png",
                            }
                            span {
                                class: "font-semibold my-auto",
                                "ORE"
                            }
                        }
                        input {
                            class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right",
                            placeholder: "0",
                            value: amount_b.cloned(),
                            oninput: move |e| {
                                let s = e.value();
                                if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                    amount_b.set(s);
                                } else {
                                    amount_b.set(s[..s.len()-1].to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}