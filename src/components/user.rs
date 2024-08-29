use std::str::FromStr;

use dioxus::prelude::*;
use ore_api::consts::BUS_ADDRESSES;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{ActivityTable, BackButton, Copyable, OreIcon, SendButton},
    hooks::{use_explorer_account_url, use_ore_balance_user, use_user_proof, use_user_transfers},
};

// TODO Not found

#[component]
pub fn User(id: String) -> Element {
    let user_id = Pubkey::from_str(&id);
    let nav = navigator();
    if user_id.is_err() {
        return rsx! {
            p {
                "Invalid user id"
            }
        };
    }

    let user_id = user_id.unwrap();
    let balance = use_ore_balance_user(user_id);
    let explorer_url = use_explorer_account_url(id.clone());
    let proof = use_user_proof(user_id);
    let title = if let Some(index) = BUS_ADDRESSES
        .iter()
        .enumerate()
        .find(|i| (*i.1).eq(&user_id))
    {
        format!("Bus {}", index.0)
    } else if user_id.eq(&ore_api::consts::TREASURY_ADDRESS) {
        "Treasury".to_string()
    } else {
        "User".to_string()
    };

    let description = if user_id.eq(&ore_api::consts::TREASURY_ADDRESS) {
        Some("This is a special program account responsible for minting new ORE tokens.")
    } else if BUS_ADDRESSES.contains(&user_id) {
        Some("This is a special program account responsible for issuing ORE to miners.")
    } else {
        None
    };

    let show_send_button = false; // title.eq("User") && user_id.ne(&pubkey);
    let container_class = "flex flex-row gap-8 justify-between py-1 sm:px-1";
    let title_class = "text-gray-300 font-medium text-sm my-auto";
    let value_class = "font-medium py-1 rounded";
    let link_class = "font-medium transition-colors -ml-2 sm:ml-0 px-2 py-1 hover-100 active-200 rounded truncate";

    rsx! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-4 -mt-3.5",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-row justify-between",
                    h2 {
                        class: "my-auto",
                        "{title}"
                    }
                    if show_send_button {
                        SendButton { to: id.clone() }
                    }
                }
                if let Some(description) = description {
                    p {
                        class: "text-sm text-gray-300 px-1",
                        "{description}"
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    if !user_id.eq(&ore_api::consts::TREASURY_ADDRESS) && !BUS_ADDRESSES.contains(&user_id) {
                        div {
                            class: "{container_class} -mr-2",
                            p {
                                class: "{title_class}",
                                "ID"
                            }
                            Copyable {
                                class: "truncate",
                                value: id.clone(),
                                Link {
                                    class: "{link_class} font-mono",
                                    to: explorer_url,
                                    new_tab: true,
                                    "{id}"
                                }
                            }
                        }
                        div {
                            class: "{container_class}",
                            p {
                                class: "{title_class}",
                                "Balance"
                            }
                            if let Some(balance) = balance.cloned() {
                                span {
                                    class: "flex flex-row gap-1",
                                    OreIcon {
                                        class: "w-4 h-4 my-auto",
                                    }
                                    p {
                                        class: "{value_class} truncate",
                                        "{balance.map(|b| b.real_number_string_trimmed()).unwrap_or_else(|_| \"0.00\".to_owned())}"
                                    }
                                }
                            } else {
                                p {
                                    class: "{value_class} w-16 h-8 loading rounded",
                                }
                            }
                        }
                        if let Some(Ok(proof)) = proof.cloned() {
                            if proof.balance.gt(&0) {
                                div {
                                    class: "{container_class}",
                                    p {
                                        class: "{title_class}",
                                        "Stake"
                                    }
                                    span {
                                        class: "flex flex-row gap-1.5",
                                        OreIcon {
                                            class: "w-4 h-4 my-auto",
                                        }
                                        p {
                                            class: "{value_class} truncate",
                                            "{amount_to_ui_amount(proof.balance, ore_api::consts::TOKEN_DECIMALS)}"
                                       }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            UserActivity {
                user_id: user_id
            }
        }
    }
}

#[component]
pub fn UserActivity(user_id: Pubkey) -> Element {
    let offset = use_signal(|| 0u64);
    let transfers = use_user_transfers(user_id, offset);
    let e = if let Some(transfers) = transfers.read().clone() {
        match transfers {
            Ok(transfers) => {
                rsx! {
                    div {
                        class: "flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start",
                        div {
                            class: "flex flex-row justify-between",
                            h2 {
                                class: "text-lg md:text-2xl font-bold",
                                "Activity"
                            }
                        }
                        ActivityTable {
                            offset,
                            transfers,
                        }
                    }
                }
            }
            _ => rsx! {},
        }
    } else {
        rsx! {
            div {
                class: "flex flex-row h-64 w-full loading rounded",
            }
        }
    };
    e
}
