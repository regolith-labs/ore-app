use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::Copyable,
    hooks::{
        use_is_onboarded, use_miner_toolbar_state, use_pubkey, use_sol_balance, IsOnboarded,
        ReadMinerToolbarState,
    },
};

pub fn MinerToolbarInsufficientFunds() -> Element {
    let mut sol_balance = use_sol_balance();
    let mut is_onboarded = use_is_onboarded();
    let toolbar_state = use_miner_toolbar_state();

    // TODO Poll balance every 3 seconds
    use_future(move || async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            if toolbar_state.is_open() {
                sol_balance.restart();
            }
        }
    });

    use_effect(move || {
        if let Some(Ok(sol_balance)) = *sol_balance.read() {
            if sol_balance.gt(&0) {
                is_onboarded.set(IsOnboarded(true));
            }
        }
    });

    rsx! {
        if toolbar_state.is_open() {
            MinerToolbarInsufficientBalanceOpen {}
        } else {
            div {
                class: "flex flex-row font-semibold justify-end w-full h-full px-4 sm:px-8 pt-5 pointer-events-none",
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
