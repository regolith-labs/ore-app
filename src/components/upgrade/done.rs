use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::signature::Signature;

use crate::{components::CheckCircleIcon, route::Route};

#[component]
pub fn UpgradeDone(signature: Signature, amount: u64) -> Element {
    rsx! {
        div {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-col gap-2",
                h2 {
                    "Success!"
                }
                p {
                    class: "text-lg",
                    "You have upgraded your ORE to v2."
                }
                p {
                    class: "text-sm text-gray-300",
                    "You can now stake it to earn a multiplier on your mining rewards."
                }
            }
            div {
                class: "flex flex-col gap-8 w-full",
                CheckCircleIcon { class: "h-12 w-12 mx-auto" }
                Link {
                    class: "font-mono text-nowrap truncate mx-auto p-2 rounded hover-100 active-200",
                    to: Route::Tx {
                        sig: signature.to_string(),
                    },
                    "{signature.to_string()}"
                }
            }
            div {
                class: "flex flex-col gap-3",
                div {
                    class: "h-full"
                }
                Link {
                    class: "w-full py-3 rounded font-semibold transition-colors text-center text-white bg-green-500 hover:bg-green-600 active:bg-green-700",
                    to: Route::Home{},
                    "Done"
                }
            }
        }
    }
}
