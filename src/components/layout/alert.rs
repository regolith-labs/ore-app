use dioxus::prelude::*;

use crate::{config::Token,components::{AlertInfoIcon, Row}, hooks::{use_wallet, use_token_balance, Wallet, MIN_SOL_BALANCE}};

pub fn Alert() -> Element {
    let wallet = use_wallet();
    let sol_balance = use_token_balance(Token::sol().mint);
    let message = format!("Warning: We suggest having at least {:.1} SOL in your wallet.", MIN_SOL_BALANCE);
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
            Row {
                class: "h-10 w-full rounded-xl p-4 bg-amber-950 flex items-center justify-start",
                gap: 2,
                AlertInfoIcon {
                    class: "h-4 w-4 shrink-0 color-black",
                } 
                span {
                    class: "text-orange-300 text-elements-lowEmphasis",
                    "{message}"
                }     
            }
        }
    } else {
        rsx! {
            {}
        }
    }    
}