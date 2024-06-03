use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

use crate::{
    components::Copyable,
    gateway::AsyncResult,
    hooks::{
        use_is_onboarded, use_miner_toolbar_state, use_pubkey, use_sol_balance, IsOnboarded,
        MinerStatus, MinerToolbarState, ReadMinerToolbarState, SolBalanceHandle,
        SolBalanceHandleOp, UpdateMinerToolbarState,
    },
};

pub fn MinerToolbarInsufficientFunds() -> Element {
    let sol_balance = use_sol_balance();
    let mut sol_balance_handle = use_context::<Signal<SolBalanceHandle>>();
    let mut toolbar_state = use_miner_toolbar_state();
    let mut is_onboarded = use_is_onboarded();

    use_effect(move || {
        if let AsyncResult::Ok(sol_balance) = *sol_balance.read() {
            if sol_balance.0.gt(&0) {
                sol_balance_handle.cancel();
                is_onboarded.set(IsOnboarded(true));
                toolbar_state.set_status(MinerStatus::Activating);
            } else {
                sol_balance_handle.restart();
            }
        }
    });

    let bg = if toolbar_state.is_open() {
        ""
    } else {
        "pointer-events-none"
    };

    if toolbar_state.is_open() {
        match *sol_balance.read() {
            AsyncResult::Ok(sol_balance) => {
                if sol_balance.0.lt(&LAMPORTS_PER_SOL.saturating_div(10)) {
                    rsx! {
                        MinerToolbarInsufficientBalanceOpen { }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        rsx! {
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

pub fn MinerToolbarInsufficientBalanceOpen() -> Element {
    let pubkey = use_pubkey();
    let solana_pay_req = solana_pay_sol_request(pubkey, 0.1);
    let qrcode = qrcode_generator::to_svg_to_string(
        solana_pay_req,
        qrcode_generator::QrCodeEcc::Low,
        192,
        None::<&str>,
    )
    .unwrap();

    rsx! {
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
                    "Scan the QR code from your Solana wallet to top up your miner."
                }
                p {
                    class: "text-sm text-gray-300",
                    "Your miner keypair is stored on your local device and can be exported at anytime from settings."
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
