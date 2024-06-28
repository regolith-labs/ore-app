use dioxus::prelude::*;

use crate::hooks::use_wallet_adapter;

#[component]
pub fn WalletAdapter() -> Element {
    rsx! {
        Render {}
        RenderPubkey {}
    }
}

#[component]
fn Render() -> Element {
    let _ = use_resource(move || async move {
        let init_wallets = eval(
            r#"
                const walletAdapter = window.OreWalletAdapter;
                console.log(walletAdapter);
                walletAdapter();
                return
            "#,
        );
        let _ = init_wallets.await;
    });
    rsx! {
        nav { id: "ore-wallet-adapter-id" }
    }
}

#[component]
fn RenderPubkey() -> Element {
    let wallet_adapter_signal = use_wallet_adapter::use_wallet_adapter();
    let maybe_wallet_adapter = *wallet_adapter_signal.read();
    let e = match maybe_wallet_adapter {
        Some(wallet_adapter) => {
            rsx! {
                div { "{wallet_adapter.pubkey.to_string()}" }
            }
        }
        None => {
            rsx! {
                div { "no pubkey yet" }
            }
        }
    };
    e
}
