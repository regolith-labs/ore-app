use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{message::Message, transaction::Transaction};
use solana_extra_wasm::program::{
    spl_associated_token_account::instruction::create_associated_token_account,
    spl_token::{self, amount_to_ui_amount},
};
use solana_sdk::compute_budget::ComputeBudgetInstruction;

use crate::{
    components::{BackButton, InvokeSignature, OreIcon},
    gateway::{ore_token_account_address, ore_token_account_address_v1},
    hooks::{
        use_gateway,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
    },
};

use super::UpgradeStep;

#[component]
pub fn UpgradeConfirm(upgrade_step: Signal<UpgradeStep>, amount: u64) -> Element {
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let wallet_adapter = use_wallet_adapter();
    let tx = use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => None,
            WalletAdapter::Connected(signer) => {
                // Build ixs
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(600_000);
                let mut ixs = vec![cu_limit_ix];

                // Create token account if necessary
                let gateway = use_gateway();
                if gateway
                    .get_token_account_ore_from_pubkey(signer)
                    .await
                    .is_err()
                {
                    ixs.push(create_associated_token_account(
                        &signer,
                        &signer,
                        &ore_api::consts::MINT_ADDRESS,
                        &spl_token::id(),
                    ));
                }

                // Build upgrade ix
                let v1_token_account_address = ore_token_account_address_v1(signer);
                let v2_token_account_address = ore_token_account_address(signer);
                ixs.push(ore_api::instruction::upgrade(
                    signer,
                    v2_token_account_address,
                    v1_token_account_address,
                    amount,
                ));

                // Build tx
                let blockhash = gateway.rpc.get_latest_blockhash().await.unwrap();
                let message = Message::new_with_blockhash(&ixs, Some(&signer), &blockhash);
                let tx = Transaction::new_unsigned(message);
                Some(tx)
            }
        }
    });

    log::info!("TX: {:?}", tx);

    if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
        upgrade_step.set(UpgradeStep::Done(sig));
    };

    rsx! {
        div {
            class: "flex flex-col h-full grow gap-12 justify-between",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        upgrade_step.borrow_mut().set(UpgradeStep::Edit);
                    }
                }
                h2 { "Confirm upgrade" }
                p {
                    class: "text-lg",
                    "Please review your upgrade information for correctness."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "Once confirmed, this transaction cannot be undone."
                }
            }
            div {
                class: "flex flex-col gap-8",
                div {
                    class: "flex flex-col gap-2",
                    p { "Upgrade" }
                    div {
                        class: "flex flex-row gap-4",
                        div {
                            p {
                                class: "text-2xl",
                                "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS_V1)} OREv1"
                            }
                        }
                        p {
                            class: "text-2xl",
                            "→"
                        }
                        div {
                            class: "flex flex-row gap-2",
                            OreIcon { class: "my-auto w-5 h-5" }
                            p {
                                class: "text-2xl",
                                "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS_V1)}"
                            }
                        }
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
