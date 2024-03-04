use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::components::Link;
use ore::{BUS_ADDRESSES, TREASURY_ADDRESS};
use ore_types::TransferType;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{Copyable, OreIcon},
    gateway::AsyncResult,
    hooks::{use_datetime, use_explorer_transaction_url, use_transfer},
    route::Route,
};

#[component]
pub fn Tx(cx: Scope, sig: String) -> Element {
    let transfer = use_transfer(cx, sig.clone());

    match transfer {
        AsyncResult::Ok(transfer) => {
            let transfer_memo = transfer.memo.unwrap_or("–".to_string());
            let title = match transfer.transfer_type {
                TransferType::Claim => "Claim",
                TransferType::Mine => "Mine",
                TransferType::Spl => "Transfer",
            };
            let amount = (transfer.amount as f64) / (10f64.powf(ore::TOKEN_DECIMALS as f64));
            let explorer_url = use_explorer_transaction_url(cx, &transfer.sig);
            let date = use_datetime(transfer.ts);
            let container_class = "flex flex-col gap-1 sm:flex-row sm:justify-between py-2 sm:px-1";
            let title_class = "opacity-50 text-sm my-auto";
            let value_class = "font-medium sm:px-2 py-1 rounded";
            let link_class = "font-medium transition-colors -ml-2 sm:ml-0 px-2 py-1 hover-100 active-200 rounded truncate";
            let from_name = if let Ok(from_address) = Pubkey::from_str(&transfer.from_address) {
                if from_address.eq(&TREASURY_ADDRESS) {
                    "Treasury".to_string()
                } else if let Some(index) = BUS_ADDRESSES
                    .iter()
                    .enumerate()
                    .find(|i| (*i.1).eq(&from_address))
                {
                    format!("Bus {:?}", index.0)
                } else {
                    transfer.from_address.clone()
                }
            } else {
                transfer.from_address.clone()
            };
            render! {
                div {
                    class: "flex flex-col gap-4 w-full",
                    h2 {
                        class: "text-lg md:text-2xl font-bold",
                        "{title}"
                    }
                    div {
                        class: "flex flex-col gap-2",
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "Signature"
                            }
                            Copyable {
                                value: transfer.sig.clone(),
                                Link {
                                    class: "{link_class} font-mono",
                                    to: "{explorer_url}",
                                    "{&transfer.sig.as_str()}"
                                }
                            }
                        }
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "To"
                            }
                            Copyable {
                                value: transfer.to_address.clone(),
                                Link {
                                    class: "{link_class} font-mono",
                                    to: Route::User { id: transfer.to_address.clone() },
                                    "{&transfer.to_address}"
                                }
                            }
                        }
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "From"
                            }
                            Copyable {
                                value: transfer.from_address.clone(),
                                Link {
                                    class: "{link_class}",
                                    to: Route::User { id: transfer.from_address.clone() },
                                    "{from_name}"
                                }
                            }
                        }
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "Amount"
                            }
                            span {
                                class: "flex flex-row gap-1",
                                OreIcon {
                                    class: "w-3.5 h-3.5 my-auto",
                                }
                                p {
                                    class: "{value_class}",
                                    "{amount}"
                                }
                            }
                        }
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "Memo"
                            }
                            p {
                                class: "{value_class}",
                                "{transfer_memo}"
                            }
                        }
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "Timestamp"
                            }
                            p {
                                class: "{value_class}",
                                "{date}"
                            }
                        }
                    }
                }
            }
        }
        AsyncResult::Loading => {
            render! {
                p {
                    "Loading"
                }
            }
        }
        _ => None,
    }
}
