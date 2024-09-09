use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, transaction::Transaction,
};
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{BackButton, InvokeSignature, OreIcon},
    gateway::{self, ore_token_account_address},
    hooks::{
        use_gateway, use_ore_balance,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
    },
};

use super::StakeStep;

#[component]
pub fn StakeConfirm(amount: u64, step: Signal<StakeStep>) -> Element {
    let mut ore_balance = use_ore_balance();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let wallet_adapter = use_wallet_adapter();

    let tx = use_resource(move || {
        async move {
            if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
                // Cu limit
                let gateway = use_gateway();
                let price = gateway::get_recent_priority_fee_estimate(true).await + 1000;
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(40_000);
                let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
                let mut ixs = vec![cu_limit_ix, cu_price_ix];
                let token_account_address = ore_token_account_address(signer);

                // Add transfer
                ixs.push(ore_api::instruction::stake(
                    signer,
                    token_account_address,
                    amount,
                ));

                // Return tx
                let mut tx = Transaction::new_with_payer(&ixs, Some(&signer));
                tx.message.recent_blockhash = gateway.rpc.get_latest_blockhash().await.unwrap();
                Some(tx)
            } else {
                None
            }
        }
    });

    if let InvokeSignatureStatus::Done(_sig) = *invoke_signature_signal.read() {
        ore_balance.restart();
        step.set(StakeStep::Done);
    };

    rsx! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        step.borrow_mut().set(StakeStep::Edit);
                    }
                }
                h2 {
                    "Confirm"
                }
                p {
                    class: "text-lg",
                    "Please review your stake information for correctness."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "Once confirmed, this transaction cannot be undone."
                }
            }
            div {
                class: "flex flex-col gap-8",
                div {
                    class: "flex flex-row gap-2.5 md:gap-4 mx-auto",
                    OreIcon {
                        class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10"
                    }
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl font-semibold",
                        "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS)}"
                    }
                }
            }
            if let Some(Some(tx)) = tx.cloned() {
                InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Confirm" }
            } else {
                p {
                    class: "font-medium text-center text-sm text-gray-300 hover:underline",
                    "Loading..."
                }
            }
        }
    }
}
