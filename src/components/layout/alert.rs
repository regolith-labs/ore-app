use dioxus::prelude::*;

use crate::{
    components::{AlertWarningIcon, Col, Row},
    config::Token,
    hooks::{use_token_balance, use_wallet, Wallet, MIN_SOL_BALANCE},
};

pub fn Alert() -> Element {
    let wallet = use_wallet();
    let sol_balance = use_token_balance(Token::sol().mint);
    let message = format!(
        "For best performance, top up with at least {:.1} SOL.",
        MIN_SOL_BALANCE
    );
    let mut show_alert = use_signal(|| false);

    use_effect(move || {
        if let Wallet::Disconnected = *wallet.read() {
            show_alert.set(false);
            return;
        }
        if let Some(Ok(token_amount)) = sol_balance.cloned() {
            let sol_amount = token_amount.ui_amount.unwrap();
            let has_low_balance = sol_amount < MIN_SOL_BALANCE;
            show_alert.set(has_low_balance);
        }
    });

    if *show_alert.read() {
        rsx! {
            Col {
                class: "h-min w-full rounded-xl px-4 py-2 bg-amber-950 flex justify-start text-left",
                Row {
                    gap: 2,
                    AlertWarningIcon {
                        class: "h-4 w-4 shrink-0 color-black mb-auto mt-1",
                    }
                    span {
                        class: "text-orange-300 text-elements-lowEmphasis my-auto",
                        span {
                            class: "font-semibold",
                            "Low balance: "
                        }
                        "{message}"
                    }

                }
            }
        }
    } else {
        rsx! {
            {}
        }
    }
}
