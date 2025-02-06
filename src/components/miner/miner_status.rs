use dioxus::prelude::*;
use ore_pool_types::Member;

use crate::{
    config::Pool, gateway::GatewayResult, hooks::{
        use_member_onchain, use_miner_is_active, use_register_db, use_register_onchain, use_transaction_status, use_wallet, Wallet
    }
};

use crate::components::{submit_transaction, TransactionStatus};

#[component]
pub fn MinerStatus(member_db: Resource<GatewayResult<Member>>, pool: Pool) -> Element {
    let wallet = use_wallet();
    let is_active = use_miner_is_active();
    let mut member_onchain = use_member_onchain(pool.address);
    let mut register_db = use_register_db(pool.url.clone());
    let register_onchain = use_register_onchain(pool.address);
    let transaction_status = use_transaction_status();

    let el = match *wallet.read() {
        Wallet::Disconnected => {
            rsx! {}
        }
        Wallet::Connected(_pubkey) => {
            match is_active.read().0 {
                false => {
                    // do nothing
                    rsx! {}
                }
                true => {
                    match &*member_db.read_unchecked() {
                        Some(Ok(_member_db)) => {
                            // start mining
                            rsx! {
                                div { "start mining" }
                            }
                        }
                        Some(Err(err)) => {
                            log::error!("{:?}", err);
                            // check for member on chain
                            match &*member_onchain.read_unchecked() {
                                Some(Ok(member_onchain)) => {
                                    log::info!("{:?}", member_onchain);
                                    // register member with the pool
                                    match &*register_db.read() {
                                        Some(Ok(_)) => {
                                            member_db.restart();
                                            rsx! {
                                                div { "restarting member db lookup" }
                                            }
                                        }
                                        Some(Err(err)) => {
                                            let err = format!("{:?}", err);
                                            rsx! {
                                                div { "{err}" }
                                            }
                                        }
                                        None => {
                                            rsx! {
                                                div { "waiting for register db" }
                                            }
                                        }
                                    }
                                }
                                Some(Err(err)) => {
                                    log::error!("{:?}", err);

                                    // register member on chain first
                                    match &*register_onchain.read() {
                                        Some(Ok(tx)) => {
                                            let tx = tx.clone();
                                            let el = if let Some(transaction_status) = transaction_status.cloned() {
                                                match transaction_status {
                                                    TransactionStatus::Waiting => {
                                                        rsx! {
                                                            div {
                                                                "waiting for register onchain signature"
                                                            }
                                                        }
                                                    }
                                                    TransactionStatus::Denied
                                                    | TransactionStatus::Timeout
                                                    | TransactionStatus::Error => {
                                                        rsx! {
                                                            div {
                                                                "error with register onchain signature"
                                                            }
                                                        }
                                                    }
                                                    TransactionStatus::Sending(attempt) => {
                                                        rsx! {
                                                            div {
                                                                "sending register onchain transaction, attempt {attempt}"
                                                            }
                                                        }
                                                    }
                                                    TransactionStatus::Done(_sig) => {
                                                        member_onchain.restart();
                                                        register_db.restart();
                                                        rsx! {
                                                            div { "restarting register db" }
                                                        }
                                                    }
                                                }
                                            } else {
                                                submit_transaction(tx.into());
                                                rsx! {}
                                            };
                                            el
                                        }
                                        Some(Err(err)) => {
                                            let err = format!("{:?}", err);
                                            rsx! {
                                                div { "{err}" }
                                            }
                                        }
                                        None => {
                                            rsx! {
                                                div { "waiting for register onchain transaction builder" }
                                            }
                                        }
                                    }
                                }
                                None => {
                                    rsx! {
                                        div { "waiting for member onchain lookup" }
                                    }
                                }
                            }
                        }
                        None => {
                            rsx! {
                                div { "waiting for member db lookup" }
                            }
                        }
                    }
                }
            }
        }
    };
    el
}
