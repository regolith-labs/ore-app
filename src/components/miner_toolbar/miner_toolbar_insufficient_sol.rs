use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
#[cfg(feature = "desktop")]
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

use crate::{
    components::{Copyable, IsToolbarOpen, MinerStatus},
    gateway::AsyncResult,
    hooks::{use_is_onboarded, use_pubkey, use_sol_balance, IsOnboarded},
};

#[component]
pub fn MinerToolbarInsufficientFunds(cx: Scope) -> Element {
    let sol_balance = use_sol_balance(cx);
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let is_onboarded = use_is_onboarded(cx);

    use_future(cx, &sol_balance, |_| {
        let miner_status = miner_status.clone();
        let is_onboarded = is_onboarded.clone();
        async move {
            if let AsyncResult::Ok(sol_balance) = sol_balance {
                if sol_balance.0.gt(&0) {
                    *is_onboarded.write() = IsOnboarded(true);
                    *miner_status.write() = MinerStatus::Activating;
                }
            }
        }
    });

    let bg = if is_toolbar_open.read().0 {
        ""
    } else {
        "pointer-events-none"
    };

    if is_toolbar_open.read().0 {
        match sol_balance {
            AsyncResult::Ok(sol_balance) => {
                if sol_balance.0.lt(&LAMPORTS_PER_SOL.saturating_div(10)) {
                    render! {
                        MinerToolbarInsufficientBalanceOpen { }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        render! {
            div {
                class: "flex flex-row font-semibold justify-end w-full h-full px-4 sm:px-8 pt-5 {bg}",
                span {
                    class: "font-semibold",
                    "Insufficient funds →"
                }
            }
        }
    }
}

#[component]
pub fn MinerToolbarInsufficientBalanceOpen(cx: Scope) -> Element {
    let pubkey = use_pubkey(cx);
    let solana_pay_req = solana_pay_sol_request(pubkey, 0.1);
    let qrcode = qrcode_generator::to_svg_to_string(
        solana_pay_req,
        qrcode_generator::QrCodeEcc::Low,
        192,
        None::<&str>,
    )
    .unwrap();

    render! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between px-4 sm:px-8 py-8",
            div {
                class: "flex flex-col gap-2",
                p {
                    class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                    "Pay transaction fees"
                }
                p {
                    class: "text-lg",
                    "Scan the QR code from your Solana wallet to fund your miner."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "Top up with 0.1 SOL to begin mining."
                }
            }
            div {
                class: "flex flex-col gap-4 sm:gap-6 md:gap-8",
                div {
                    class: "text-center w-48 h-48 bg-gray-100 mx-auto",
                    dangerous_inner_html: "{qrcode}",
                }
                Copyable {
                    class: "mx-auto max-w-full",
                    value: pubkey.to_string(),
                    p {
                        class: "rounded p-2 font-mono font-medium truncate",
                        "{pubkey}"
                    }
                }
            }
            a {
                // TODO Get referal code
                href: "https://www.coinbase.com/price/solana",
                target: "_blank",
                class: "font-medium text-center text-sm text-gray-300 dark:text-gray-700 hover:underline",
                "Help! I don't have any SOL."
            }
        }
    }
}

fn solana_pay_sol_request(pubkey: Pubkey, amount: f64) -> String {
    format!(
        "solana:{}?amount={}&label=Ore&message=Topping%20up%20Ore%20miner",
        pubkey, amount
    )
}
