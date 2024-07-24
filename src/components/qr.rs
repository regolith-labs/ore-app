use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::BackButton,
    hooks::use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

use super::wallet_adapter;

#[component]
pub fn Qr() -> Element {
    let nav = use_navigator();
    let wallet_adapter = use_wallet_adapter();

    let qrcode = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => None,
            WalletAdapter::Connected(address) => {
                let solana_pay_req = solana_pay_url(address);
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
                    // p {
                    //     class: "text-lg",
                    //     "Pay ORE from your miner."
                    // }
                    // p {
                    //     class: "text-sm text-gray-300",
                    //     "This will transfer ORE to your wallet and decrease your mining multiplier."
                    // }
                }
            }
            div {
                class: "flex flex-col gap-8",
                if let Some(qrcode) = qrcode.cloned() {
                    if let Some(qrcode) = qrcode {
                        div {
                            class: "text-center w-48 h-48 bg-gray-100 mx-auto",
                            dangerous_inner_html: "{qrcode}",
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

            }
        }
    }
}

fn solana_pay_url(pubkey: Pubkey) -> String {
    // format!(
    //     "solana:{}?amount={}&label=Ore&message=Topping%20up%20Ore%20miner",
    //     pubkey, amount
    // )
    format!("solana:{}?&label=ORE", pubkey)
}
