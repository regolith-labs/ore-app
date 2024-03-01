use dioxus::prelude::*;

use crate::{
    components::{Activity, Balance},
    hooks::{use_ore_balance, use_pubkey},
};

#[component]
pub fn Home(cx: Scope) -> Element {
    let pubkey = use_pubkey(cx);
    let (balance, balance_) = use_ore_balance(cx, pubkey);

    render! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            Balance { balance: balance }
            Activity {}
        }
    }
}
