use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use futures::StreamExt;
use solana_sdk::pubkey::Pubkey;

use crate::{
    gateway::{AccountNotificationParams, AccountSubscribe, AccountSubscribeGateway, GatewayError},
    hooks::{use_wallet, GetPubkey},
};

pub type FromWss = Signal<FromWssMsg>;
pub type ToWss = Coroutine<ToWssMsg>;
#[derive(Debug)]
pub enum FromWssMsg {
    Init,
    Subscription(SubId),
    Notif(AccountNotificationParams),
}
#[derive(Debug, Clone, Copy)]
pub enum ToWssMsg {
    Subscribe(Pubkey),
    Unsubscribe(SubId),
}
type SubId = u64;
pub fn use_wss() -> (FromWss, ToWss) {
    let from = use_context::<Signal<FromWssMsg>>();
    let to = use_coroutine_handle::<ToWssMsg>();
    (from, to)
}

pub fn use_wss_provider() {
    // listen to wallet
    let wallet = use_wallet();
    // init from wss
    let mut from_wss = use_context_provider(|| Signal::new(FromWssMsg::Init));
    // init to wss
    let _to_wss = use_coroutine(move |mut rx: UnboundedReceiver<ToWssMsg>| async move {
        if let Err(err) = async {
            // connect to wss
            let pubkey = wallet.pubkey()?;
            let wss = AccountSubscribeGateway::connect().await?;
            let wss = Arc::new(Mutex::new(wss));

            // clone for subscriber task
            let wss_for_sub = wss.clone();
            let mut from_wss_for_sub = from_wss.clone();

            // listen for messages from subscribers
            spawn(async move {
                while let Some(msg) = rx.next().await {
                    log::info!("to wss: {:?}", msg);
                    match msg {
                        ToWssMsg::Subscribe(pubkey) => {
                            let mut wss_guard = match wss_for_sub.lock() {
                                Ok(guard) => guard,
                                Err(e) => {
                                    log::error!("failed to lock wss: {:?}", e);
                                    continue;
                                }
                            };
                            match wss_guard.subscribe(pubkey.to_string().as_str()).await {
                                Ok(sub_id) => {
                                    log::info!("sub id: {}", sub_id);
                                    from_wss_for_sub.set(FromWssMsg::Subscription(sub_id));
                                }
                                Err(err) => {
                                    log::error!("failed to wss sub: {:?}", err);
                                }
                            }
                        }
                        ToWssMsg::Unsubscribe(sub_id) => {
                            let mut wss_guard = match wss_for_sub.lock() {
                                Ok(guard) => guard,
                                Err(e) => {
                                    log::error!("failed to lock wss: {:?}", e);
                                    continue;
                                }
                            };
                            if let Err(err) = wss_guard.unsubscribe(sub_id).await {
                                log::error!("failed to wss unsub: {:?}", err);
                            }
                        }
                    }
                }
            });

            // clone for notification task
            let wss_for_notif = wss.clone();
            let mut from_wss_for_notif = from_wss.clone();

            // listen for messages from rpc server
            spawn(async move {
                loop {
                    let notification = {
                        let mut wss_guard = match wss_for_notif.lock() {
                            Ok(guard) => guard,
                            Err(e) => {
                                log::error!("failed to lock wss: {:?}", e);
                                continue;
                            }
                        };
                        match wss_guard.next_notification().await {
                            Ok(msg) => {
                                log::info!("wss msg from rpc server: {:?}", msg);
                                Some(msg)
                            }
                            Err(err) => {
                                log::info!("wss err from rpc server: {:?}", err);
                                None
                            }
                        }
                    };
                    // process notification outside the lock to avoid holding it too long
                    if let Some(notif) = notification {
                        from_wss_for_notif.set(FromWssMsg::Notif(notif.params));
                    }
                }
            });

            Ok::<_, GatewayError>(())
        }
        .await
        {
            log::error!("{:?}", err);
        }
    });
}

// pub fn use_wss_provider() -> Signal<GatewayResult<UiTokenAmount>> {
//     let wallet = use_wallet();
//     let mut balance = use_signal(|| Err::<UiTokenAmount, _>(GatewayError::AccountNotFound));
//     use_resource(move || async move {
//         if let Err(e) = async {
//             let pubkey = wallet.pubkey()?;
//             let mut wss = AccountSubscribeGateway::connect().await?;
//             let id = wss.subscribe(pubkey.to_string().as_str()).await?;
//             log::info!("sub id: {}", id);
//             loop {
//                 log::info!("starting loop");
//                 match wss.next_notification().await {
//                     Ok(msg) => {
//                         log::info!("{:?}", msg);
//                     }
//                     Err(err) => {
//                         log::error!("{:?}", err);
//                         break;
//                     }
//                 }
//             }
//             log::info!("broke");
//             Ok::<(), GatewayError>(())
//         }
//         .await
//         {
//             log::error!("WebSocket subscription error: {:?}", e);
//         }
//     });
//     balance
// }
