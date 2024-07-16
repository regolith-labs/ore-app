use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::program::{
    spl_associated_token_account::create_associated_token_account,
    spl_memo,
    spl_token::{self, amount_to_ui_amount},
};
use solana_sdk::{compute_budget::ComputeBudgetInstruction, transaction::Transaction};

use crate::{
    components::{BackButton, InvokeSignature, OreIcon, Spinner},
    gateway::{self, ore_token_account_address},
    hooks::{
        use_gateway, use_ore_balance,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
    },
};

use super::SendStep;

#[component]
pub fn SendConfirm(
    send_step: Signal<SendStep>,
    amount: u64,
    recipient: Pubkey,
    memo: String,
) -> Element {
    let mut is_busy = use_signal(|| false);
    let mut ore_balance = use_ore_balance();
    let gateway = use_gateway();
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let wallet_adapter = use_wallet_adapter();
    // let memo_bytes = memo.into_bytes().clone();

    let tx = use_resource(move || {
        async move {
            if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
                // Cu limit
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);
                let mut ixs = vec![cu_limit_ix];
                let from_token_account = ore_token_account_address(signer);
                let to_token_account = ore_token_account_address(recipient);

                // Add create ata ix
                let gateway = use_gateway();
                if let Ok(Some(_)) = gateway.get_token_account(&to_token_account).await {
                } else {
                    ixs.push(create_associated_token_account(
                        &signer,
                        &recipient,
                        &ore_api::consts::MINT_ADDRESS,
                    ));
                }

                // Add transfer
                ixs.push(
                    spl_token::instruction::transfer(
                        &spl_token::ID,
                        &from_token_account,
                        &to_token_account,
                        &signer,
                        &[&signer],
                        amount,
                    )
                    .unwrap(),
                );

                // Add memo
                // ixs.push(spl_memo::build_memo(&memo_bytes, &[&signer]));

                // Return tx
                let mut tx = Transaction::new_with_payer(&ixs, Some(&signer));
                tx.message.recent_blockhash = gateway.rpc.get_latest_blockhash().await.unwrap();
                Some(tx)
            } else {
                None
            }
        }
    });

    use_future(move || async move {
        if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
            // TODO
        };
    });

    rsx! {
        div {
            class: "flex flex-col h-full grow gap-12",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        send_step.borrow_mut().set(SendStep::Edit);
                    }
                }
                h2 {
                    "Confirm transfer"
                }
                p {
                    class: "text-lg",
                    "Please review your transfer information for correctness."
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
                    p {
                        "Amount"
                    }
                    div {
                        class: "flex flex-row gap-2",
                        OreIcon {
                            class: "my-auto w-5 h-5"
                        }
                        p {
                            class: "text-2xl",
                            "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS)}"
                       }
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        "To"
                    }
                    p {
                        class: "text-2xl",
                        "{recipient.to_string()}"
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        "Memo"
                    }
                    p {
                        class: "text-2xl",
                        "{memo}"
                    }
                }
            }
            div {
                class: "flex flex-col mt-auto sm:flex-row gap-2",
                if let Some(Some(tx)) = tx.cloned() {
                    InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Create account" }
                } else {
                    p {
                        class: "font-medium text-center text-sm text-gray-300 hover:underline",
                        "Loading..."
                    }
                }
                // button {
                //     class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                //     disabled: *is_busy.read(),
                //     onclick: move |_| {
                //         let gateway = gateway.clone();
                //         let memo = memo.clone();
                //         is_busy.set(true);
                //         spawn(async move {
                //             // TODO Use wallet adapter
                //             // match gateway.transfer_ore(amount, recipient, memo).await {
                //             //     Ok(sig) => {
                //             //         ore_balance.restart();
                //             //         is_busy.set(false);
                //             //         send_step.set(SendStep::Done);
                //             //     }
                //             //     Err(err) => {
                //             //         // TODO Handle error
                //             //         is_busy.set(false);
                //             //         log::error!("Failed to send: {:?}", err);
                //             //     }
                //             // }
                //         });
                //     },
                //     if *is_busy.read() {
                //         Spinner {
                //             class: "mx-auto"
                //         }
                //     } else {
                //         "Confirm"
                //     }
                // }
            }
        }
    }
}
