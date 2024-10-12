use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, transaction::Transaction,
};
use solana_extra_wasm::program::{
    spl_associated_token_account::get_associated_token_address, spl_token::amount_to_ui_amount,
};

use crate::{
    components::{BackButton, InvokeSignature, OreIcon},
    gateway::{self, GatewayError},
    hooks::{
        use_balance, use_gateway,
        use_wallet_adapter::{use_wallet_adapter, InvokeSignatureStatus, WalletAdapter},
    },
    pool::POOL_ADDRESS,
};

use super::{Stake, StakeStep};

#[component]
pub fn StakeConfirm(amount: u64, step: Signal<StakeStep>, stake: Stake) -> Element {
    let mut balance = use_balance(stake.mint, stake.decimals);
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    let wallet_adapter = use_wallet_adapter();

    let tx = use_resource(move || {
        async move {
            if let WalletAdapter::Connected(signer) = *wallet_adapter.read() {
                let gateway = use_gateway();
                let mut ixs = vec![];
                let mut cu_additional: u32 = 0;
                // Check if user has joined pool yet
                let (member_pda, _) = ore_pool_api::state::member_pda(signer, POOL_ADDRESS);
                let member_data = gateway.rpc.get_account_data(&member_pda).await;
                let join_pool_ix = match member_data {
                    Ok(_) => None,
                    Err(_) => {
                        let ix = ore_pool_api::sdk::join(signer, POOL_ADDRESS, signer);
                        cu_additional += 10_000;
                        Some(ix)
                    }
                };
                // Check if user has a share account opened
                let (share_pda, _) =
                    ore_pool_api::state::share_pda(signer, POOL_ADDRESS, stake.mint);
                let share_data = gateway.rpc.get_account_data(&share_pda).await;
                let open_share_ix = match share_data {
                    Ok(_) => None,
                    Err(_) => {
                        let ix = ore_pool_api::sdk::open_share(signer, stake.mint, POOL_ADDRESS);
                        cu_additional += 10_000;
                        Some(ix)
                    }
                };
                // Cu limit
                let cu_price = gateway::get_recent_priority_fee_estimate(true).await + 1000;
                let cu_limit = 40_000 + cu_additional;
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(cu_limit);
                let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(cu_price);
                ixs.push(cu_limit_ix);
                ixs.push(cu_price_ix);
                if let Some(ix) = join_pool_ix {
                    ixs.push(ix);
                }
                if let Some(ix) = open_share_ix {
                    ixs.push(ix);
                }

                // Add stake
                let token_account_address = get_associated_token_address(&signer, &stake.mint);
                ixs.push(ore_pool_api::sdk::stake(
                    signer,
                    stake.mint,
                    POOL_ADDRESS,
                    token_account_address,
                    amount,
                ));

                // Return tx
                let mut tx = Transaction::new_with_payer(&ixs, Some(&signer));
                tx.message.recent_blockhash = gateway.rpc.get_latest_blockhash().await?;
                Ok(tx)
            } else {
                Err(GatewayError::WalletAdapterDisconnected)
            }
        }
    });

    if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
        log::info!("{:?}", sig);
        balance.restart();
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
                        "{amount_to_ui_amount(amount, stake.decimals)}"
                    }
                }
            }
            if let Some(Ok(tx)) = tx.cloned() {
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
