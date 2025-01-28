use dioxus::prelude::*;
use steel::Pubkey;

use crate::{components::{stake_form::common::{WithdrawButton, WithdrawDetails}, Col, Row}, config::{BoostMeta, LISTED_TOKENS}, hooks::use_token_balance};


#[component]
pub fn PairWithdrawForm(class: Option<String>, boost_meta: BoostMeta) -> Element {
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

    let balance = use_token_balance(boost_meta.pair_mint);
    let token = LISTED_TOKENS.get(&boost_meta.pair_mint).unwrap();

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
                            // button {
                            //     class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                            //     onclick: move |_| {
                            //         if let Some(Ok(balance)) = balance.read().as_ref() {
                            //             withdraw_amount.set(balance.ui_amount.unwrap_or(0.0).to_string());
                            //         }
                            //     },
                            //     "Max"
                            // }
                        }
                        Row {
                            gap: 4,
                            Row {
                                class: "my-auto",
                                gap: 2,
                                img {
                                    class: "w-8 h-8 rounded-full shrink-0",
                                    src: "{token.image}"
                                }
                                span {
                                    class: "font-semibold my-auto",
                                    "{token.ticker}"
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
            // WithdrawDetails {}
            WithdrawButton {
                enabled: enabled,
            }
        }
    }
}