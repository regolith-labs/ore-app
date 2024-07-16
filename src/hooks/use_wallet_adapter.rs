use base64::Engine;
use dioxus::prelude::*;
use ore_api::consts::{TOKEN_DECIMALS, TOKEN_DECIMALS_V1};
use solana_client_wasm::solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_client_wasm::solana_sdk::signature::Signature;
use solana_client_wasm::solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, transaction::Transaction,
};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;
use solana_extra_wasm::program::spl_associated_token_account::instruction::create_associated_token_account;
use solana_extra_wasm::program::{spl_memo, spl_token};

use crate::gateway::{
    ore_token_account_address, ore_token_account_address_v1, GatewayError, GatewayResult,
    CU_LIMIT_UPGRADE,
};
use crate::hooks::UiTokenAmountDefault;

use super::use_gateway;

pub fn use_wallet_adapter() -> Signal<WalletAdapter> {
    use_context::<Signal<WalletAdapter>>()
}

pub fn use_wallet_adapter_provider() {
    let mut signal = use_context_provider(|| Signal::new(WalletAdapter::Disconnected));
    let mut eval = eval(
        r#"
            window.addEventListener("ore-pubkey", (event) => {
                dioxus.send(event.detail.pubkey);
            });
        "#,
    );
    spawn(async move {
        while let Ok(json_val) = eval.recv().await {
            let pubkey_result: Result<Pubkey, serde_json::Error> = serde_json::from_value(json_val);
            match pubkey_result {
                Ok(pubkey) => signal.set(WalletAdapter::Connected(pubkey)),
                Err(_) => signal.set(WalletAdapter::Disconnected),
            }
        }
    });
}

// we only have one resource per hook
// until we update to latest dioxus from git
// so will have to pack many future here
// in a big result if we want more reactive async data
pub fn use_ore_balances() -> Resource<Option<Balances>> {
    let gateway = use_gateway();
    let signal = use_wallet_adapter();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            match *signal.read() {
                WalletAdapter::Connected(pubkey) => {
                    let token_account_address_v1 = ore_token_account_address_v1(pubkey);
                    let token_account_address_v2 = ore_token_account_address(pubkey);
                    let balance_v1 = gateway
                        .rpc
                        .get_token_account_balance(&token_account_address_v1)
                        .await
                        .unwrap_or(UiTokenAmount::default(TOKEN_DECIMALS_V1));
                    let balance_v2 = gateway
                        .rpc
                        .get_token_account_balance(&token_account_address_v2)
                        .await
                        .unwrap_or(UiTokenAmount::default(TOKEN_DECIMALS));
                    Some(Balances {
                        v1: balance_v1,
                        v2: balance_v2,
                    })
                }
                WalletAdapter::Disconnected => None,
            }
        }
    })
}

pub fn invoke_signature(tx: Transaction, mut signal: Signal<InvokeSignatureStatus>) {
    signal.set(InvokeSignatureStatus::Waiting);
    let mut eval = eval(
        r#"
        let msg = await dioxus.recv();
        let signed = await window.OreTxSigner({b64: msg});
        dioxus.send(signed);
        "#,
    );
    match bincode::serialize(&tx) {
        Ok(vec) => {
            let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
            let res = eval.send(serde_json::Value::String(b64));
            match res {
                Ok(()) => {
                    spawn(async move {
                        let res = eval.recv().await;
                        match res {
                            Ok(serde_json::Value::String(string)) => {
                                let gateway = use_gateway();
                                let decode_res = base64::engine::general_purpose::STANDARD
                                    .decode(string)
                                    .ok()
                                    .and_then(|buffer| bincode::deserialize(&buffer).ok());
                                let rpc_res = match decode_res {
                                    Some(tx) => {
                                        log::info!("Sending: {:?}", tx);
                                        let x = gateway.rpc.send_transaction(&tx).await;
                                        log::info!("Sent: {:?}", x);
                                        x.ok()
                                    }
                                    None => {
                                        log::info!("error decoding tx");
                                        None
                                    }
                                };
                                log::info!("Dec: {:?}", rpc_res);
                                match rpc_res {
                                    Some(sig) => {
                                        log::info!("sig: {}", sig);
                                        signal.set(InvokeSignatureStatus::Done(sig));
                                    }
                                    None => {
                                        log::info!("error sending tx");
                                        signal.set(InvokeSignatureStatus::DoneWithError)
                                    }
                                }
                            }
                            _ => {
                                log::info!("err recv val");
                                signal.set(InvokeSignatureStatus::DoneWithError)
                            }
                        };
                    });
                }
                Err(_err) => {
                    log::info!("err sending val");
                    signal.set(InvokeSignatureStatus::DoneWithError)
                }
            }
        }
        Err(err) => {
            log::info!("err serializing tx: {}", err);
            signal.set(InvokeSignatureStatus::DoneWithError)
        }
    };
}

#[derive(PartialEq)]
pub enum InvokeSignatureStatus {
    Start,
    Waiting,
    DoneWithError,
    Done(Signature),
}

#[derive(Clone)]
pub struct Balances {
    pub v1: UiTokenAmount,
    pub v2: UiTokenAmount,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WalletAdapter {
    Connected(Pubkey),
    Disconnected,
}

impl WalletAdapter {
    pub async fn build_open_tx(&self) -> GatewayResult<Transaction> {
        match *self {
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
    }

    pub async fn build_upgrade_tx(&self, amount: u64) -> GatewayResult<Transaction> {
        match *self {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(pubkey) => {
                let gateway = use_gateway();
                // v2 token account may or may not exist
                // we'll build an ix to create this token account if needed
                // for the wallet adapter to sign
                let v2_token_account_result =
                    gateway.get_token_account_ore_from_pubkey(pubkey).await;
                // the v1 token account *must* exist
                // return immediately if not
                let v1_token_account = gateway.get_token_account_ore_from_pubkey_v1(pubkey).await?;
                // build upgrade ix
                let build_upgrade_ore_ix = |v2_token_account_address: &Pubkey| -> Instruction {
                    ore_api::instruction::upgrade(
                        pubkey,
                        *v2_token_account_address,
                        v1_token_account,
                        amount,
                    )
                };
                // build ixs
                let ixs = match v2_token_account_result {
                    // v2 token account exists
                    Ok(token_account_address) => {
                        // compute limit ix
                        let cu_limit_ix =
                            ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_UPGRADE);
                        // upgrade ix
                        let upgrade_ix = build_upgrade_ore_ix(&token_account_address);
                        // pack ixs
                        vec![cu_limit_ix, upgrade_ix]
                    }
                    Err(_) => {
                        // compute limit ix
                        // TODO: exact amount for creating token account
                        let cu_limit_ix =
                            ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
                        // create token account ix
                        let create_token_account_ix = create_associated_token_account(
                            &pubkey,
                            &pubkey,
                            &ore_api::consts::MINT_ADDRESS,
                            &spl_token::id(),
                        );
                        // upgrade ix
                        let upgrade_ix = build_upgrade_ore_ix(&ore_token_account_address(pubkey));
                        // pack ixs
                        vec![cu_limit_ix, create_token_account_ix, upgrade_ix]
                    }
                };
                let blockhash = gateway.rpc.get_latest_blockhash().await?;
                let message =
                    Message::new_with_blockhash(ixs.as_slice(), Some(&pubkey), &blockhash);
                let tx = Transaction::new_unsigned(message);
                Ok(tx)
            }
        }
    }

    async fn _build_transfer_tx(
        &self,
        to: &Pubkey,
        amount: u64,
        memo: String,
    ) -> GatewayResult<Transaction> {
        match *self {
            WalletAdapter::Disconnected => Err(GatewayError::WalletAdapterDisconnected),
            WalletAdapter::Connected(pubkey) => {
                // from token account must exist
                let from_token_account = ore_token_account_address(pubkey);
                // to token account might exist
                // so pack ix to create account if not
                let to_token_account = ore_token_account_address(*to);
                let maybe_create_to_token_account_ix =
                    WalletAdapter::_build_create_token_account_ix(&pubkey, to).await;
                // build ixs
                let memo_ix = spl_memo::build_memo(&memo.into_bytes(), &[&pubkey]);
                let transfer_ix = spl_token::instruction::transfer(
                    &spl_token::id(),
                    &from_token_account,
                    &to_token_account,
                    &to_token_account,
                    &[&pubkey],
                    amount,
                )
                .map_err(GatewayError::from)?;
                let ixs = match maybe_create_to_token_account_ix {
                    Some(create_token_account_ix) => {
                        vec![memo_ix, create_token_account_ix, transfer_ix]
                    }
                    None => {
                        vec![memo_ix, transfer_ix]
                    }
                };
                // build transaction
                Ok(Transaction::new_with_payer(ixs.as_slice(), Some(&pubkey)))
            }
        }
    }

    async fn _build_create_token_account_ix(payer: &Pubkey, owner: &Pubkey) -> Option<Instruction> {
        let gateway = use_gateway();
        let token_account_address = ore_token_account_address(*owner);
        match gateway.rpc.get_token_account(&token_account_address).await {
            Ok(Some(_)) => None,
            _ => {
                let ix = create_associated_token_account(
                    payer,
                    owner,
                    &ore_api::consts::MINT_ADDRESS,
                    &spl_token::id(),
                );
                Some(ix)
            }
        }
    }
}
