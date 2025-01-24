use dioxus::prelude::*;
use steel::Pubkey;

use crate::components::{Col, Row};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum StakeTab {
    Deposit,
    Withdraw,
}

pub(crate) fn StakeDetails() -> Element {
    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Estimated yield",
                value: "1 ORE / day"
            }
            DetailLabel {
                title: "Transaction fee",
                value: "0.00005 SOL"
            }
        }
    }
}

pub(crate) fn WithdrawDetails() -> Element {
    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Transaction fee",
                value: "0.00005 SOL"
            }
        }
    }
}

#[component]
pub(crate) fn DetailLabel(title: String, value: String) -> Element {
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
pub(crate) fn StakeButton(enabled: Signal<bool>) -> Element {
    let colors = if *enabled.read() {
        "controls-primary"
    } else {
        "bg-controls-disabled text-on-onDisabled"
    };
    rsx! {
        button {
            class: "h-12 w-full rounded-full {colors}",
            disabled: !*enabled.read(),
            onclick: move |_| {
                // TODO: Implement staking logic
            },
            span {
                class: "mx-auto my-auto font-semibold",
                "Submit"
            }
        }
    }
}

#[component]
pub(crate) fn WithdrawButton(enabled: Signal<bool>) -> Element {
    let colors = if *enabled.read() {
        "controls-primary"
    } else {
        "bg-controls-disabled text-on-onDisabled"
    };
    rsx! {
        button {
            class: "h-12 w-full rounded-full {colors}",
            disabled: !*enabled.read(),
            onclick: move |_| {
                // TODO: Implement withdraw logic
            },
            span {
                class: "mx-auto my-auto font-semibold",
                "Submit"
            }
        }
    }
}
#[component]
pub(crate) fn StakeInputs(
    mint: Pubkey,
    amount_a: Signal<String>,
    amount_b: Signal<String>,
    pair_deposit: Signal<bool>,
) -> Element {
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
                gap: if *pair_deposit.read() { 4 } else { 0 },
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
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
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
                if *pair_deposit.read() {
                    Row {
                        class: "border-t border-gray-800 pt-4",
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
                            class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                            placeholder: "0",
                            r#type: "number",
                            inputmode: "decimal",
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
