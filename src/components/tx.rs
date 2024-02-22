use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::components::Link;
use ore::{BUS_ADDRESSES, TREASURY_ADDRESS};
use ore_types::TransferType;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{components::OreIcon, gateway::AsyncResult, hooks::use_transfer, route::Route};

// TODO Not found

#[component]
pub fn Tx(cx: Scope, sig: String) -> Element {
    let transfer = use_transfer(cx, sig.clone());

    match transfer {
        AsyncResult::Ok(transfer) => {
            let transfer_memo = transfer.memo.unwrap_or("–".to_string());
            let transfer_type = match transfer.transfer_type {
                TransferType::Claim => "Claim",
                TransferType::Mine => "Mine",
                TransferType::Spl => "Spl",
            };
            let amount = (transfer.amount as f64) / (10f64.powf(ore::TOKEN_DECIMALS as f64));
            let explorer_url = format!("https://explorer.solana.com/tx/{}", transfer.sig);
            let container_class = "flex flex-row justify-between py-2 px-1";
            let title_class = "text-gray-300";
            let value_class = "font-medium";
            let link_class = "font-medium hover:underline";
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
                    class: "flex flex-col gap-2",
                    h2 {
                        class: "text-lg md:text-2xl font-bold",
                        "Transfer"
                    }
                    div {
                        class: "{container_class}",
                        p {
                            class: "{title_class}",
                            "ID"
                        }
                        Link {
                            class: "{link_class}",
                            to: "{explorer_url}",
                            "{&transfer.sig.as_str()[..32]}"
                        }
                    }
                    div {
                        class: "{container_class}",
                        p {
                            class: "{title_class}",
                            "From"
                        }
                        Link {
                            class: "{link_class}",
                            to: Route::User { id: transfer.from_address.clone() },
                            "{from_name}"
                        }
                    }
                    div {
                        class: "{container_class}",
                        p {
                            class: "{title_class}",
                            "To"
                        }
                        Link {
                            class: "{link_class}",
                            to: Route::User { id: transfer.to_address.clone() },
                            "{transfer.to_address}"
                        }
                    }
                    div {
                        class: "{container_class}",
                        p {
                            class: "{title_class}",
                            "Amount"
                        }
                        span {
                            class: "flex flex-row gap-1.5",
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
                            "Type"
                        }
                        p {
                            class: "{value_class}",
                            "{transfer_type}"
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
                            "{transfer.ts}"
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
