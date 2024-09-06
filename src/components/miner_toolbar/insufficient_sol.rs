use std::ops::Div;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, message::Message, native_token::sol_to_lamports,
    pubkey, pubkey::Pubkey, transaction::Transaction,
};
use web_time::Duration;

use crate::{
    components::{BackButton, InfoIcon, InvokeSignature},
    gateway::{self, escrow_pubkey, GatewayError, GatewayResult},
    hooks::{
        use_escrow, use_gateway, use_proof,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
    },
};

const TOP_UP_AMOUNT: f64 = 0.02; // In SOL (~$2)
const COLLECTION_ADDRESS: Pubkey = pubkey!("tHCCE3KWKx8i8cDjX2DQ3Z7EMJkScAVwkfxdWz8SqgP");

#[component]
pub fn MinerToolbarTopUpOpen(escrow_balance: Resource<GatewayResult<u64>>) -> Element {
    let wallet_adapter = use_wallet_adapter();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let nav = use_navigator();

    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(signer) => {
                let gateway = use_gateway();
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(100_000);
                let amount = sol_to_lamports(TOP_UP_AMOUNT);
                let ix_1 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &escrow_pubkey(signer),
                    amount,
                );
                let ix_2 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &COLLECTION_ADDRESS,
                    amount.div(100), // 1% fee
                );
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let ixs = vec![cu_limit_ix, ix_1, ix_2];
                let msg = Message::new_with_blockhash(ixs.as_slice(), Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(msg);
                Ok(tx)
            }
        }
    });

    let _ = use_resource(move || async move {
        if let InvokeSignatureStatus::Done(_sig) = *invoke_signature_signal.read() {
            if let WalletAdapter::Connected(_signer) = *wallet_adapter.read() {
                async_std::task::sleep(Duration::from_millis(2000)).await;
                escrow_balance.restart();
            }
        };
        ()
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                        "Top up"
                    }
                    p {
                        class: "text-lg",
                        "Fund your account to pay for blockchain transaction fees."
                    }
                    // p {
                    //     class: "text-sm text-gray-300",
                    //     "This will fund your account to automate mining."
                    // }
                }
            }
            div {
                class: "flex flex-col gap-4",
                if let Some(Ok(tx)) = tx.cloned() {
                    InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Top up" }
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
                    class: "font-medium text-center py-2 text-sm text-gray-300 hover:underline",
                    "Help! I don't have any SOL."
                }
            }
        }
    }
}

pub fn CreateAccountPage() -> Element {
    let wallet_adapter = use_wallet_adapter();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let mut proof = use_proof();
    let nav = use_navigator();

    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(signer) => {
                let gateway = use_gateway();
                let price = gateway::get_recent_priority_fee_estimate(false).await;
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);
                let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
                let amount = sol_to_lamports(TOP_UP_AMOUNT);
                let ix_1 = ore_relayer_api::instruction::open_escrow(signer, signer);
                // TODO This is commented out because users are currently manually signing (not escrow)
                // let ix_2 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                //     &signer,
                //     &escrow_pubkey(signer),
                //     amount,
                // );
                let ix_3 = solana_client_wasm::solana_sdk::system_instruction::transfer(
                    &signer,
                    &COLLECTION_ADDRESS,
                    amount.div(100), // 1% fee
                );
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let ixs = vec![cu_limit_ix, cu_price_ix, ix_1, ix_3];
                let msg = Message::new_with_blockhash(ixs.as_slice(), Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(msg);
                Ok(tx)
            }
        }
    });

    let _ = use_resource(move || async move {
        if let InvokeSignatureStatus::Done(_sig) = *invoke_signature_signal.read() {
            if let WalletAdapter::Connected(_signer) = *wallet_adapter.read() {
                async_std::task::sleep(Duration::from_millis(2000)).await;
                proof.restart();
            }
        };
        ()
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                        "New account"
                    }
                    p {
                        class: "text-lg",
                        "Open a new account to start mining."
                    }
                    p {
                        class: "text-sm text-gray-300",
                        "This will open a new account on the Solana blockchain to secure your mining rewards."
                    }
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
                    class: "font-medium text-center py-2 text-sm text-gray-300 hover:underline",
                    "Help! I don't have any SOL."
                }
            }
        }
    }
}

pub fn MigrateAccountPage() -> Element {
    let wallet_adapter = use_wallet_adapter();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let mut escrow = use_escrow();
    let mut proof = use_proof();
    let nav = use_navigator();

    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(signer) => {
                let gateway = use_gateway();
                let price = gateway::get_recent_priority_fee_estimate(false).await;
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_000_000);
                let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
                let ix = ore_relayer_api::instruction::migrate(signer, signer, signer);
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let ixs = vec![cu_limit_ix, cu_price_ix, ix];
                let msg = Message::new_with_blockhash(ixs.as_slice(), Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(msg);
                Ok(tx)
            }
        }
    });

    let _ = use_resource(move || async move {
        if let InvokeSignatureStatus::Done(_sig) = *invoke_signature_signal.read() {
            if let WalletAdapter::Connected(_signer) = *wallet_adapter.read() {
                async_std::task::sleep(Duration::from_millis(2000)).await;
                escrow.restart();
                proof.restart();
            }
        };
        ()
    });

    // Reduced fees
    // Preparation for future features
    //

    rsx! {
        div {
            class: "flex flex-col h-full w-full grow gap-12 sm:gap-16 justify-between",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                        "Migrate account"
                    }
                    p {
                        class: "text-lg",
                        "To continue mining, please migrate your account to our new systems."
                    }
                    p {
                        class: "text-sm text-gray-300",
                        "An older version this website relied on infrastructure that has now been deprecated."
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                div {
                    class: "flex flex-row gap-4",
                    InfoIcon {
                        class: "w-5 h-5 shrink-0"
                    }
                    p {
                        class: "font-bold text-lg",
                        "Info"
                    }
                }
                ul {
                    class: "flex flex-col gap-2",
                    li {
                        "To prepare for new features such as mining pools, we have deprecated a backend transaction relayer system this app used to rely on."
                    }
                    li {
                        "To continue mining, users must migrate to a fully self-custodied account. The migration will require only 1 transaction and can be completed in 5 seconds or less."
                    }
                    li {
                        "Your stake balance will automatically be migrated to the new account."
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                if let Some(Ok(tx)) = tx.cloned() {
                    InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Migrate" }
                } else {
                    p {
                        class: "font-medium text-center text-sm text-gray-300 hover:underline",
                        "Loading..."
                    }
                }
            }
        }
    }
}
