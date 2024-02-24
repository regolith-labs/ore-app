use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::components::Link;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{ActivityTable, Copyable, OreIcon},
    gateway::AsyncResult,
    hooks::{use_explorer_account_url, use_ore_balance, use_user_transfers},
};

// TODO Not found

#[component]
pub fn User(cx: Scope, id: String) -> Element {
    let user_id = Pubkey::from_str(id);

    if user_id.is_err() {
        return render! {
            p {
                "Invalid user id"
            }
        };
    }

    let user_id = user_id.unwrap();
    let (balance, _) = use_ore_balance(cx, user_id);
    let explorer_url = use_explorer_account_url(cx, id);

    let container_class = "flex flex-row justify-between py-2 px-1";
    let title_class = "text-gray-300";
    let value_class = "font-medium px-2 py-1 rounded";
    let link_class = "font-medium hover-100 active-200 px-2 py-1 rounded transition-colors";

    render! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-2",
                div {
                    class: "flex flex-row gap-8",
                    div {
                        class: "rounded-full w-16 h-16 bg-gray-100 dark:bg-gray-700",
                    }
                    h2 {
                        class: "text-lg md:text-2xl font-bold my-auto",
                        "User"
                    }
                }
                div {
                    class: "pl-2",
                    div {
                        class: "{container_class}",
                        p {
                            class: "{title_class}",
                            "ID"
                        }
                        Copyable {
                            value: id.clone(),
                            Link {
                                class: "{link_class} font-mono",
                                to: "{explorer_url}",
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
                        match balance {
                            AsyncResult::Ok(balance) => {
                                render! {
                                    span {
                                        class: "flex flex-row gap-1.5",
                                        OreIcon {
                                            class: "w-3.5 h-3.5 my-auto",
                                        }
                                        p {
                                            class: "{value_class}",
                                            "{balance.real_number_string_trimmed()}"
                                        }
                                    }
                                }
                            }
                            _ => {
                                render! {
                                    p {
                                        class: "{value_class} w-16 h-8 loading rounded",
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
pub fn UserActivity(cx: Scope, user_id: Pubkey) -> Element {
    let offset = use_state(cx, || 0u64);
    let (transfers, has_more) = use_user_transfers(cx, *user_id, offset);
    match transfers {
        AsyncResult::Ok(transfers) => {
            render! {
                div {
                    class: "flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start",
                    div {
                        class: "flex flex-row justify-between",
                        h2 {
                            class: "text-lg md:text-2xl font-bold",
                            "Activity"
                        }
                    }
                    ActivityTable{
                        offset: offset,
                        transfers: transfers,
                        has_more: has_more
                    }
                }
            }
        }
        _ => {
            render! {
                div {
                    class: "flex flex-row h-64 w-full loading rounded",
                }
            }
        }
    }
}
