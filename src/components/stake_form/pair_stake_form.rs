use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{Col, Row}, config::LISTED_TOKENS_BY_TICKER, hooks::use_token_balance
};
use super::common::*;

#[component]
pub fn PairStakeForm(class: Option<String>, mint: Pubkey) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| StakeTab::Deposit);
    rsx! {
        Col {
            class: "{class}",
            StakeTabs {
                tab: tab
            }
            match *tab.read() {
                StakeTab::Deposit => rsx! {
                    PairDepositForm {
                        mint: mint
                    }
                },
                StakeTab::Withdraw => rsx! {
                    PairWithdrawForm {
                        mint: mint
                    }
                }
            }
        }
    }
}

#[component]
fn PairWithdrawForm(class: Option<String>, mint: Pubkey) -> Element {
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
    let image = LISTED_TOKENS_BY_TICKER
        .get(&token)
        .map(|token| token.image.clone())
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
fn PairDepositForm(class: Option<String>, mint: Pubkey) -> Element {
    let class = class.unwrap_or_default();
    let stake_amount_a = use_signal::<String>(|| "".to_owned());
    let stake_amount_b = use_signal::<String>(|| "".to_owned());
    let mut enabled = use_signal(|| false);
    let mut pair_deposit = use_signal(|| true);

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
            // Row {
            //     class: "px-1",
            //     label {
            //         class: "flex items-center gap-2 text-sm text-elements-lowEmphasis cursor-pointer",
            //         input {
            //             r#type: "checkbox",
            //             checked: *pair_deposit.read(),
            //             oninput: move |_| {
            //                 pair_deposit.set(!pair_deposit.cloned())
            //             }
            //         }
            //         "Deposit as pair"
            //     }
            // }
            // StakeDetails {}
            SubmitButton {
                enabled: enabled,
                onclick: move |_| {
                    // TODO: Implement staking logic
                }
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
