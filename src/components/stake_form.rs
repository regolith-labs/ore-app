use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{Col, Row},
    hooks::{use_token_balance, ASSETS},
};

#[component]
pub fn StakeForm(class: Option<String>, mint: Pubkey) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| StakeTab::Deposit);

    rsx! {
        Col {
            class: "{class}",
            // gap: 2,
            StakeTabs {
                tab: tab
            }
            match *tab.read() {
                StakeTab::Deposit => rsx! {
                    DepositForm {
                        mint: mint
                    }
                },
                StakeTab::Withdraw => rsx! {
                    WithdrawForm {
                        mint: mint
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum StakeTab {
    Deposit,
    Withdraw,
}

#[component]
fn StakeTabs(tab: Signal<StakeTab>) -> Element {
    let deposit_class = if *tab.read() == StakeTab::Deposit {
        "flex-1 h-12 transition-colors text-elements-highEmphasis border-b-2 border-white"
    } else {
        "flex-1 h-12 transition-colors text-elements-lowEmphasis hover:bg-surface-elevated"
    };

    let withdraw_class = if *tab.read() == StakeTab::Withdraw {
        "flex-1 h-12 transition-colors text-elements-highEmphasis border-b-2 border-white"
    } else {
        "flex-1 h-12 transition-colors text-elements-lowEmphasis hover:bg-surface-elevated"
    };

    rsx! {
        Row {
            class: "w-full mb-4",
            gap: 1,
            button {
                class: "{deposit_class}",
                onclick: move |_| tab.set(StakeTab::Deposit),
                "Deposit"
            }
            button {
                class: "{withdraw_class}",
                onclick: move |_| tab.set(StakeTab::Withdraw),
                "Withdraw"
            }
        }
    }
}

#[component]
pub fn WithdrawForm(class: Option<String>, mint: Pubkey) -> Element {
    let class = class.unwrap_or_default();
    let mut withdraw_amount = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);

    use_effect(move || {
        let amount_str = withdraw_amount.cloned();

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

    let balance = use_token_balance(mint);
    let token = "SOL".to_string(); // This should be dynamically determined based on mint
    let image = ASSETS
        .get(&token)
        .map(|asset| asset.image.clone())
        .unwrap_or_else(|| "icon.png".to_string());

    rsx! {
        Col {
            gap: 4,
            Col {
                class: "p-1 bg-surface-elevated rounded-lg border border-gray-800 {class}",
                gap: 4,
                Row {
                    class: "w-full p-3",
                    gap: 2,
                    Col {
                        class: "flex-1",
                        Row {
                            class: "justify-between mb-2",
                            span {
                                class: "text-elements-midEmphasis my-auto pl-1",
                                "Withdraw"
                            }
                            button {
                                class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                                onclick: move |_| {
                                    if let Some(Ok(balance)) = balance.read().as_ref() {
                                        withdraw_amount.set(balance.ui_amount.unwrap_or(0.0).to_string());
                                    }
                                },
                                "Max"
                            }
                        }
                        Row {
                            gap: 4,
                            Row {
                                class: "my-auto",
                                gap: 2,
                                img {
                                    class: "w-8 h-8 rounded-full shrink-0",
                                    src: "{image}"
                                }
                                span {
                                    class: "font-semibold my-auto",
                                    "{token}"
                                }
                            }
                            input {
                                class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                placeholder: "0",
                                r#type: "number",
                                inputmode: "decimal",
                                value: "{withdraw_amount}",
                                oninput: move |evt| {
                                    let s = evt.value();
                                    if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                        withdraw_amount.set(s);
                                    } else {
                                        withdraw_amount.set(s[..s.len()-1].to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            WithdrawDetails {}
            WithdrawButton {
                enabled: enabled,
            }
        }
    }
}

#[component]
pub fn DepositForm(class: Option<String>, mint: Pubkey) -> Element {
    let class = class.unwrap_or_default();
    let stake_amount_a = use_signal::<String>(|| "".to_owned());
    let stake_amount_b = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);
    let mut pair_deposit = use_signal(|| false);

    use_effect(move || {
        let amount_str_a = stake_amount_a.cloned();
        let amount_str_b = stake_amount_b.cloned();

        if !*pair_deposit.read() {
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
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
                StakeInputs {
                    mint: mint,
                    amount_a: stake_amount_a,
                    amount_b: stake_amount_b,
                    pair_deposit: pair_deposit
                }
            }
            Row {
                class: "px-1",
                label {
                    class: "flex items-center gap-2 text-sm text-elements-lowEmphasis cursor-pointer",
                    input {
                        r#type: "checkbox",
                        checked: *pair_deposit.read(),
                        oninput: move |_| {
                            pair_deposit.set(!pair_deposit.cloned())
                        }
                    }
                    "Deposit as pair"
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

fn WithdrawDetails() -> Element {
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
fn WithdrawButton(enabled: Signal<bool>) -> Element {
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
fn StakeInputs(
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

