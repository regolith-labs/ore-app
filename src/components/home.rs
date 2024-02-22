use dioxus::prelude::*;

use crate::{
    components::{Activity, Balance, ClaimModal, IsToolbarOpen},
    hooks::{use_ore_balance, use_pubkey},
};

#[component]
pub fn Home(cx: Scope) -> Element {
    let pubkey = use_pubkey(cx);
    let (balance, balance_) = use_ore_balance(cx, pubkey);

    render! {
        div {
            class: "flex flex-col gap-16",
            Balance { balance: balance }
            Activity {}
            ClaimModal {
                balance_handle: balance_
            }
        }
    }
}

#[component]
pub fn ToolbarClose(cx: Scope) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let opacity = if is_toolbar_open.read().0 {
        "opacity-80"
    } else {
        "opacity-0 pointer-events-none"
    };
    render! {
        button {
            class: "absolute transition-opacity flex flex-row left-0 top-0 h-screen w-screen bg-black {opacity}",
            onclick: |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(false);
            }
        }
    }
}
