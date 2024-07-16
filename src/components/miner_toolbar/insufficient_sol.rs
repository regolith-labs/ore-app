use dioxus::prelude::*;
use gloo::net::websocket::Message;
use miner_toolbar::try_start_mining;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, hash::Hash, pubkey::Pubkey, transaction::Transaction,
};
use wallet_adapter::InvokeSignature;

use crate::{
    components::{miner_toolbar, wallet_adapter, Copyable},
    hooks::{
        use_gateway, use_is_onboarded, use_miner_toolbar_state, use_pubkey, use_sol_balance,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
        IsOnboarded, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
};

#[component]
pub fn MinerToolbarInsufficientFunds(miner: Signal<Miner>) -> Element {
    let toolbar_state = use_miner_toolbar_state();
    rsx! {
        if toolbar_state.is_open() {
            MinerToolbarOpenAccount {
                miner
            }
        } else {
            div {
                class: "flex flex-row font-semibold justify-end w-full h-full px-4 sm:px-8 pt-5 pointer-events-none",
                span {
                    class: "font-semibold",
                    "Create account →"
                }
            }
        }
    }
}

#[component]
pub fn MinerToolbarOpenAccount(miner: Signal<Miner>) -> Element {
    let wallet_adapter = use_wallet_adapter();
    let mut toolbar_state = use_miner_toolbar_state();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);

    let tx = use_resource(move || async move {
        if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
            let gateway = use_gateway();
            let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);
            let ix = ore_relayer_api::instruction::open_escrow(signer, signer);
            let ixs = vec![cu_limit_ix, ix];
            let mut tx = Transaction::new_with_payer(&ixs, Some(&signer));
            tx.message.recent_blockhash = gateway.rpc.get_latest_blockhash().await.unwrap();
            Some(tx)
        } else {
            None
        }
    });

    use_future(move || async move {
        if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
            try_start_mining(miner, &mut toolbar_state).await;
        };
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between px-4 sm:px-8 py-8",
            div {
                class: "flex flex-col gap-2",
                p {
                    class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                    "Create an account"
                }
                p {
                    class: "text-lg",
                    "Open a new account to start mining ORE."
                }
                p {
                    class: "text-sm text-gray-300",
                    "This account will secure your progress and miner rewards."
                }
            }
            div {
                class: "flex flex-col gap-4",
                if let Some(Some(tx)) = tx.cloned() {
                    InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Create account" }
                } else {
                    p {
                        class: "font-medium text-center text-sm text-gray-300 hover:underline",
                        "Loading..."
                    }
                }
                a {
                    // TODO Get referal code
                    href: "https://www.coinbase.com/price/solana",
                    target: "_blank",
                    class: "font-medium text-center text-sm text-gray-300 hover:underline",
                    "Help! I don't have any SOL."
                }
            }
        }
    }
}
