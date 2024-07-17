use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, message::Message, transaction::Transaction,
};
use web_time::Duration;

use crate::{
    components::{
        miner_toolbar::try_start_mining,
        wallet_adapter::{self, InvokeSignature},
    },
    gateway::{proof_pubkey, GatewayError},
    hooks::{
        use_escrow, use_gateway, use_miner_toolbar_state,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
        MinerStatus, MinerStatusMessage, ReadMinerToolbarState, UpdateMinerToolbarState,
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
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let mut escrow = use_escrow();
    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(signer) => {
                let gateway = use_gateway();
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);
                let ix = ore_relayer_api::instruction::open_escrow(signer, signer);
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let ixs = vec![cu_limit_ix, ix];
                let msg = Message::new_with_blockhash(ixs.as_slice(), Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(msg);
                Ok(tx)
            }
        }
    });

    let _ = use_resource(move || async move {
        if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
            if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
                let gateway = use_gateway();
                async_std::task::sleep(Duration::from_millis(1000)).await;
                if let Ok(new_escrow) = gateway.get_escrow(signer).await {
                    escrow.set(new_escrow);
                }
            }
        };
        ()
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
                if let Some(Ok(tx)) = tx.cloned() {
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
