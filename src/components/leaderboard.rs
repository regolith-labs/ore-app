use std::{rc::Rc, str::FromStr};

use dioxus::prelude::*;
use dioxus_router::prelude::*;
#[cfg(feature = "desktop")]
use solana_account_decoder::parse_token::UiTokenAccount;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "web")]
use solana_extra_wasm::account_decoder::parse_token::UiTokenAccount;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::OreIcon,
    gateway::{AsyncResult, Gateway},
    hooks::use_gateway,
    route::Route,
};

#[component]
pub fn Leaderboard(cx: Scope) -> Element {
    let token_accounts = use_state(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);

    use_future(cx, (), |_| {
        let gateway = gateway.clone();
        let token_accounts = token_accounts.clone();
        async move {
            token_accounts.set(AsyncResult::Ok(fetch_top_accounts(gateway).await));
        }
    });

    render! {
        div {
            class: "flex flex-col gap-4",
            h2 {
                class: "text-lg md:text-2xl font-bold",
                "Top holders"
            }
            match token_accounts.get() {
                AsyncResult::Ok(token_accounts) => {
                    render! {
                        LeaderboardTable {
                            token_accounts: token_accounts
                        }
                    }
                }
                _ => render! {
                    div {
                        class: "flex flex-row w-full h-32 loading rounded",
                    }
                }
            }
        }
    }
}

#[component]
pub fn LeaderboardTable<'a>(cx: Scope, token_accounts: &'a Vec<UiTokenAccount>) -> Element {
    if token_accounts.is_empty() {
        render! {
            p {
                class: "text-sm text-gray-300 py-2 px-1",
                "No transactions found"
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-col gap-4",
                div {
                    class: "flex flex-col gap-0 justify-start grow h-full",
                    LeaderboardTableHeader {}
                    for (i, token_account) in token_accounts.iter().enumerate() {
                        render! {
                            TokenBalanceRow {
                                i: i + 1,
                                token_account: token_account
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TokenBalanceRow<'a>(cx: Scope, i: usize, token_account: &'a UiTokenAccount) -> Element {
    let owner = &token_account.owner;
    let owner_abbr = &owner[..5];
    let amount = &token_account.token_amount.ui_amount_string;
    render! {
        Link {
            to: Route::User { id: owner.clone() },
            class: "flex flex-row shrink w-full justify-between rounded px-2 py-2 hover-100 active-200 transition-colors",
            p {
                class: "w-32 text-left",
                "{i}"
            }
            p {
                class: "w-full text-left font-mono font-medium hidden sm:block",
                "{owner}"
            }
            p {
                class: "w-full text-left font-mono font-medium block sm:hidden",
                "{owner_abbr}"
            }
            div {
                class: "flex flex-row gap-1 w-full text-right justify-end",
                OreIcon {
                    class: "my-auto w-4 h-4"
                }
                p {
                    class: "font-medium",
                    "{amount}"
                }
            }
        }
    }
}

#[component]
pub fn LeaderboardTableHeader(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row shrink w-full justify-between rounded px-2 py-2 transition-colors text-xs font-medium text-gray-300",
            p {
                class: "text-left w-32",
                "#"
            }
            p {
                class: "text-left w-full",
                "User"
            }
            p {
                class: "text-right w-full",
                "Amount"
            }
            // p {
            //     class: "text-right w-1/4",
            //     "Time"
            // }
        }
    }
}

// TODO Impl clone in solana-client-wasm to get this
// #[cached]
async fn fetch_top_accounts(gateway: Rc<Gateway>) -> Vec<UiTokenAccount> {
    let mut fetched_accounts = vec![];
    if let Ok(balances) = gateway.get_token_largest_accounts(&ore::MINT_ADDRESS).await {
        for balance in balances {
            if let Ok(pubkey) = Pubkey::from_str(&balance.address) {
                if let Ok(Some(token_account)) = gateway.get_token_account(&pubkey).await {
                    fetched_accounts.push(token_account);
                }
            }
        }
    }
    fetched_accounts
}
