use dioxus::prelude::*;

use crate::{
    components::Row,
    hooks::{on_transaction_done, use_sol_balance, use_wallet, Wallet},
};

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>) -> Element {
    let wallet = use_wallet();
    let mut pubkey = use_signal(|| "missing pubkey".to_string());
    let mut sol_balance = use_sol_balance();
    let sol_balance_string = use_signal::<String>(|| match sol_balance.cloned() {
        Some(Ok(str)) => str
            .ui_amount
            .map(|f| f.to_string())
            .unwrap_or("0".to_string()),
        _ => "0".to_string(),
    });
    use_effect(move || {
        if let Wallet::Connected(pk) = *wallet.read() {
            pubkey.set(pk.to_string());
        }
    });
    on_transaction_done(move |_sig| {
        sol_balance.restart();
    });
    rsx! {
        div {
            class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            style: "padding-left: 20px; padding-right: 20px;",
            onclick: move |e| e.stop_propagation(),
            // "TODO: Wallet address + copy button"
            div {
                "{pubkey}"
            }
            Row {
                span { class: "mr-2", "SOL:" }
                span { "{sol_balance_string}" }
            }
        }
    }
}
