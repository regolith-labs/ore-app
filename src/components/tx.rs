use std::str::FromStr;

use dioxus::prelude::*;
use ore_api::consts::{BUS_ADDRESSES, TREASURY_ADDRESS};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{BackButton, Copyable, OreIcon},
    hooks::{use_datetime, use_explorer_transaction_url, use_transfer},
    route::Route,
};

#[component]
pub fn Tx(sig: String) -> Element {
    let nav = navigator();
    let transfer = use_transfer(sig.clone());
    let e = if let Some(transfer) = transfer.read().clone() {
        match transfer {
            Ok(transfer) => {
                let transfer_memo = transfer.memo.unwrap_or("–".to_string());
                let explorer_url = use_explorer_transaction_url(transfer.sig.clone());
                let date = use_datetime(transfer.ts);
                let container_class = "flex gap-8 flex-row justify-between py-2 sm:px-1";
                let title_class = "text-gray-300 font-medium text-sm my-auto";
                let value_class = "font-medium py-1 rounded";
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
                rsx! {
                    div {
                        class: "flex flex-col gap-4 w-full -mt-3.5",
                        BackButton {
                            onclick: move |_| {
                                nav.go_back()
                            }
                        }
                        p {
                            class: "text-3xl sm:text-4xl font-bold",
                            "Transaction"
                        }
                        div {
                            class: "flex flex-col gap-1",
                            div {
                                class: "{container_class}",
                                p {
                                    class: "{title_class}",
                                    "Signature"
                                }
                                Copyable {
                                    class: "truncate",
                                    value: transfer.sig.clone(),
                                    Link {
                                        class: "{link_class} font-mono",
                                        to: explorer_url,
                                        new_tab: true,
                                        "{&transfer.sig.as_str()}"
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
                                    class: "flex flex-row gap-1.5",
                                    OreIcon {
                                        class: "w-4 h-4 my-auto",
                                    }
                                    p {
                                        class: "{value_class}",
                                        "{amount_to_ui_amount(transfer.amount as u64, ore_api::consts::TOKEN_DECIMALS)}"
                                    }
                                }
                            }
                            if let Some(difficulty) = transfer.difficulty {
                                div {
                                    class: "{container_class}",
                                    p {
                                        class: "{title_class}",
                                        "Difficulty"
                                    }
                                    p {
                                        class: "{value_class}",
                                        "{difficulty}"
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
                                    class: "truncate",
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
                                    "To"
                                }
                                Copyable {
                                    class: "truncate",
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
            _ => rsx! {},
        }
    } else {
        rsx! {
            p {
                "Loading"
            }
        }
    };

    e
}
