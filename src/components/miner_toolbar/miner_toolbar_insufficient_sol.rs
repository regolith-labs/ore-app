use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
#[cfg(feature = "desktop")]
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

use crate::{
    components::{Copyable, IsToolbarOpen, MinerStatus},
    gateway::AsyncResult,
    hooks::{use_pubkey, use_sol_balance},
};

#[component]
pub fn MinerToolbarInsufficientFunds(cx: Scope) -> Element {
    let sol_balance = use_sol_balance(cx);
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();

    use_future(cx, &sol_balance, |_| {
        let miner_status = miner_status.clone();
        async move {
            if let AsyncResult::Ok(sol_balance) = sol_balance {
                if sol_balance.gt(&0) {
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
                if sol_balance.lt(&LAMPORTS_PER_SOL.saturating_div(10)) {
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
                class: "flex flex-row font-semibold justify-end w-full h-full px-8 pt-5 {bg}",
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
            class: "flex flex-col grow gap-8 justify-between p-8 bg-white",
            div {
                class: "flex flex-col gap-3",
                h1 {
                    "Pay transaction fees"
                }
                p {
                    class: "text-black text-lg",
                    "Scan the QR code with your wallet to top up your miner."
                }
                p {
                    class: "text-gray-300 text-sm",
                    "Your miner requires at least 0.1 SOL to begin mining."
                }
            }
            div {
                class: "flex flex-col gap-8",
                div {
                    class: "text-center w-48 h-48 bg-gray-100 mx-auto",
                    dangerous_inner_html: "{qrcode}",
                }
                Copyable {
                    class: "mx-auto",
                    value: pubkey.to_string(),
                    p {
                        class: "rounded p-2 font-mono font-medium",
                        "{pubkey}"
                    }
                }
            }
            a {
                // TODO Get referal code
                href: "https://coinbase.com",
                target: "_blank",
                class: "font-medium text-center text-sm text-gray-300 hover:underline",
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
