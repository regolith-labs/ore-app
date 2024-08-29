use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::BackButton,
    hooks::use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

use super::Copyable;

#[component]
pub fn Pay() -> Element {
    let nav = use_navigator();
    let wallet_adapter = use_wallet_adapter();
    let mut amount: Signal<Option<String>> = use_signal(|| None);

    let qrcode = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => None,
            WalletAdapter::Connected(address) => {
                let solana_pay_req = solana_pay_url(address, amount.cloned());
                let qrcode = qrcode_generator::to_svg_to_string(
                    solana_pay_req,
                    qrcode_generator::QrCodeEcc::Low,
                    192,
                    None::<&str>,
                )
                .unwrap();
                Some(qrcode)
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        "Pay"
                    }
                    p {
                        class: "text-lg",
                        "Scan the code to pay the connected wallet."
                    }
                    p {
                        class: "text-sm text-gray-300",
                        "This transaction will be executed and settled on the Solana blockchain."
                    }
                }
            }
            div {
                class: "flex flex-col gap-8",
                if let Some(qrcode) = qrcode.cloned() {
                    if let Some(qrcode) = qrcode {
                        div {
                            class: "flex flex-col gap-8",
                            div {
                                class: "text-center w-48 h-48 bg-gray-100 mx-auto",
                                dangerous_inner_html: "{qrcode}",
                            }
                            if let WalletAdapter::Connected(pubkey) = *wallet_adapter.read() {
                                Copyable {
                                    class: "break-all mx-auto text-center",
                                    implicit: true,
                                    value: pubkey.to_string(),
                                    p {
                                        class: "font-mono my-auto px-1",
                                        "{pubkey}"
                                    }
                                }
                            }
                        }
                    } else {
                        p {
                            class: "mx-auto",
                            "No wallet connected"
                        }
                    }
                } else {
                   div {
                       class: "loading w-48 h-48 mx-auto",
                   }
                }
            }
            div {
                class: "flex flex-row gap-8 justify-between",
                p {
                    class: "text-sm font-semibold text-gray-300 font-medium my-auto",
                    "Amount"
                }
                div {
                    class: "flex flex-row flex-shrink h-min gap-1 shrink my-auto",
                    input {
                        class: "bg-transparent text-right h-10 px-1 mb-auto font-semibold",
                        step: 0.01,
                        min: 0,
                        r#type: "number",
                        placeholder: "0",
                        value: "{amount.cloned().unwrap_or(\"\".to_string())}",
                        oninput: move |e| {
                            amount.set(Some(e.value()));
                        }
                    }
                }
            }
        }
    }
}

fn solana_pay_url(pubkey: Pubkey, amount: Option<String>) -> String {
    // format!(
    //     "solana:{}?amount={}&label=Ore&message=Topping%20up%20Ore%20miner",
    //     pubkey, amount
    // )
    match amount {
        Some(amount) => format!(
            "solana:{}?&amount={}&spl-token={}&label=ORE",
            pubkey, amount, MINT_ADDRESS
        ),
        None => format!("solana:{}?&spl-token={}&label=ORE", pubkey, MINT_ADDRESS),
    }
}
