use dioxus::prelude::*;

use crate::hooks::{
    use_member_db, use_member_onchain, use_register_onchain, use_wallet_status, WalletStatus, POOLS,
};

use super::{invoke_signature, InvokeSignatureStatus};

#[component]
pub fn Miner(is_gold: Signal<bool>) -> Element {
    let pool = POOLS.first().unwrap();

    let member_db = use_member_db(pool.url.clone());
    let member_onchain = use_member_onchain(pool.address);
    let register_onchain = use_register_onchain(pool.address);

    let wallet_status = use_wallet_status();

    let el = match *wallet_status.read() {
        WalletStatus::Disconnected => {
            // if wallet is not connected, do nothing
            rsx! {}
        }
        WalletStatus::Connected(_) => {
            match *is_gold.read() {
                false => {
                    // do nothing
                    rsx! {}
                }
                true => {
                    match &*member_db.read_unchecked() {
                        Some(Ok(_member_db)) => {
                            match *is_gold.read() {
                                true => {
                                    // start mining
                                    rsx! {}
                                }
                                false => {
                                    // waiting
                                    rsx! {}
                                }
                            }
                        }
                        Some(Err(err)) => {
                            log::error!("{:?}", err);
                            // check for member on chain
                            match &*member_onchain.read_unchecked() {
                                Some(Ok(member_onchain)) => {
                                    log::info!("{:?}", member_onchain);
                                    // register member with the pool
                                    rsx! {}
                                }
                                Some(Err(err)) => {
                                    log::error!("{:?}", err);
                                    // register member on chain first
                                    match &*register_onchain.read() {
                                        Some(Ok(tx)) => {
                                            let invoke_signature_status =
                                                use_signal(|| InvokeSignatureStatus::Start);
                                            invoke_signature(tx.clone(), invoke_signature_status);
                                            rsx! {}
                                        }
                                        Some(Err(err)) => {
                                            log::error!("{:?}", err);
                                            rsx! {}
                                        }
                                        None => {
                                            rsx! {}
                                        }
                                    }
                                }
                                None => {
                                    rsx! {}
                                }
                            }
                        }
                        None => {
                            rsx! {}
                        }
                    }
                }
            }
        }
    };
    el
}
