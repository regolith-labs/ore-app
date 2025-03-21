use dioxus::prelude::*;
use futures::{
    channel::mpsc::{self, Receiver, Sender},
    FutureExt, SinkExt, StreamExt,
};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::gateway::{
    AccountNotificationParams, AccountSubscribe, AccountSubscribeGateway, GatewayError,
};
use crate::time;

pub type FromWss = Signal<FromWssMsg>;
pub type ToWss = Coroutine<ToWssMsg>;
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FromWssMsg {
    Init,
    Subscription(SubRequestId, SubId),
    ReSubscription(SubRequestId, SubId),
    Notif(AccountNotificationParams),
}
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ToWssMsg {
    Subscribe(SubRequestId, Pubkey),
    Unsubscribe(SubId),
}
pub type SubId = u64;
pub type SubRequestId = u64;

/// Information needed to re-subscribe.
#[derive(Clone)]
struct SubscriptionInfo {
    pubkey: Pubkey,
    sub_id: Option<SubId>,
}

/// Shared state: mapping from a subscription request ID to its details.
type ActiveSubs = Arc<Mutex<HashMap<SubRequestId, SubscriptionInfo>>>;

/// Two way channel backed by a WebSocket
/// for subscribing to notifications from the RPC server.
pub(super) fn use_wss() -> (FromWss, ToWss) {
    let from = use_context::<Signal<FromWssMsg>>();
    let to = use_coroutine_handle::<ToWssMsg>();
    (from, to)
}

/// Provider that initializes the WebSocket channels.
pub fn use_wss_provider() {
    // Init from wss signal.
    let mut from_wss = use_context_provider(|| Signal::new(FromWssMsg::Init));
    // Create shared state for active subscriptions.
    let active_subs: ActiveSubs = Arc::new(Mutex::new(HashMap::new()));
    // Init to wss coroutine.
    let _to_wss = use_coroutine(move |mut rx: UnboundedReceiver<ToWssMsg>| {
        let active_subs = active_subs.clone();
        async move {
            if let Err(err) = async {
                // Create a channel for sending commands to the WebSocket worker.
                let (cmd_tx, cmd_rx) = mpsc::channel::<WssCommand>(10);

                // Spawn the WebSocket worker task that owns the connection exclusively,
                // passing the shared active subscriptions state.
                spawn(wss_worker(cmd_rx, from_wss.clone(), active_subs.clone()));

                // Handle UI commands and forward them to the worker.
                while let Some(msg) = rx.next().await {
                    match msg {
                        ToWssMsg::Subscribe(request_id, pubkey) => {
                            // One-shot channel for the subscription ID response.
                            let (sub_resp_tx, mut sub_resp_rx) = mpsc::channel::<SubId>(1);

                            // Send the subscribe command to the worker.
                            if let Err(e) = cmd_tx
                                .clone()
                                .send(WssCommand::Subscribe(request_id, pubkey, sub_resp_tx))
                                .await
                            {
                                log::error!("Failed to send subscribe command: {:?}", e);
                                continue;
                            }

                            // Wait for the subscription ID response.
                            if let Some(sub_id) = sub_resp_rx.next().await {
                                from_wss.set(FromWssMsg::Subscription(request_id, sub_id));
                            }
                        }
                        ToWssMsg::Unsubscribe(sub_id) => {
                            // Send the unsubscribe command to the worker.
                            if let Err(e) =
                                cmd_tx.clone().send(WssCommand::Unsubscribe(sub_id)).await
                            {
                                log::error!("Failed to send unsubscribe command: {:?}", e);
                            }
                        }
                    }
                }

                Ok::<_, GatewayError>(())
            }
            .await
            {
                log::error!("{:?}", err);
            }
        }
    });
}

/// WebSocket worker that owns the connection exclusively.
/// It implements reconnection logic and uses shared state to manage re-subscriptions.
async fn wss_worker(
    mut cmd_rx: Receiver<WssCommand>,
    mut from_wss: Signal<FromWssMsg>,
    active_subs: ActiveSubs,
) {
    loop {
        log::info!("Attempting to connect to WebSocket...");
        // Try connecting to the WebSocket.
        let mut wss = match AccountSubscribeGateway::connect().await {
            Ok(wss) => {
                log::info!("WebSocket connected");
                wss
            }
            Err(e) => {
                log::error!("Failed to connect to WebSocket: {:?}", e);
                // Wait a bit before retrying.
                time::sleep(5_000).await;
                continue;
            }
        };

        // Re-subscribe to active subscriptions.
        {
            let mut active = match active_subs.lock() {
                Ok(active) => active,
                Err(err) => {
                    log::error!("Failed to lock active_subs: {:?}", err);
                    continue;
                }
            };
            for (&request_id, sub_info) in active.iter_mut() {
                match wss
                    .subscribe(sub_info.pubkey.to_string().as_str(), request_id)
                    .await
                {
                    Ok(new_sub_id) => {
                        sub_info.sub_id = Some(new_sub_id);
                        from_wss.set(FromWssMsg::ReSubscription(request_id, new_sub_id));
                    }
                    Err(err) => {
                        log::error!(
                            "Failed to re-subscribe for {:?}: {:?}",
                            sub_info.pubkey,
                            err
                        );
                    }
                }
            }
        }

        // Create a channel for notifications.
        let (notification_tx, mut notification_rx) = mpsc::channel(10);

        // Spawn a task to listen for notifications from the WebSocket
        let _notification_task = spawn(async move {
            while let Some(notif) = notification_rx.next().await {
                from_wss.set(FromWssMsg::Notif(notif));
            }
        });

        // Process commands and notifications until an error occurs.
        loop {
            futures::select! {
                // Process commands coming from the UI.
                cmd = cmd_rx.next() => {
                    match cmd {
                        Some(WssCommand::Subscribe(request_id, pubkey, resp_tx)) => {
                            match wss.subscribe(pubkey.to_string().as_str(), request_id).await {
                                Ok(new_sub_id) => {
                            // Update active subscriptions state.
                            {
                                match active_subs.lock() {
                                    Ok(mut active) => {
                                        active.insert(request_id, SubscriptionInfo {
                                            pubkey,
                                            sub_id: Some(new_sub_id),
                                        });
                                    },
                                    Err(err) => {
                                        log::error!("Failed to lock active_subs: {:?}", err);
                                    }
                                }
                            }
                                    if let Err(e) = resp_tx.clone().send(new_sub_id).await {
                                        log::error!("Failed to send subscription ID response: {:?}", e);
                                    }
                                }
                                Err(err) => {
                                    log::error!("Failed to subscribe: {:?}", err);
                                    // Optionally break to force a reconnect.
                                }
                            }
                        }
                        Some(WssCommand::Unsubscribe(sub_id)) => {
                            if let Err(err) = wss.unsubscribe(sub_id).await {
                                log::error!("Failed to unsubscribe: {:?}", err);
                            } else {
                                // Remove the subscription from shared state.
                                match active_subs.lock() {
                                    Ok(mut active) => {
                                        if let Some((&req_id, _)) = active.iter().find(|(_, info)| info.sub_id == Some(sub_id)) {
                                            active.remove(&req_id);
                                        }
                                    },
                                    Err(err) => {
                                        log::error!("Failed to lock active_subs: {:?}", err);
                                    }
                                }
                            }
                        }
                        None => {
                            // Command channel closed; exit the worker.
                            return;
                        }
                    }
                }

                // Process notifications from the WebSocket.
                notification = wss.next_notification().fuse() => {
                    match notification {
                        Ok(notification) => {
                            if let Err(e) = notification_tx.clone().send(notification.params).await {
                                log::error!("Failed to forward notification: {:?}", e);
                            }
                        }
                        Err(e) => {
                            log::error!("WebSocket notification error: {:?}", e);
                            // Break out of the inner loop to attempt a reconnection.
                            break;
                        }
                    }
                }

            }
        }
        log::info!("WebSocket connection lost, reconnecting in 5 seconds...");
        time::sleep(5_000).await;
    }
}

/// Internal message types for the WebSocket worker.
#[derive(Debug)]
enum WssCommand {
    Subscribe(SubRequestId, Pubkey, Sender<SubId>),
    Unsubscribe(SubId),
}
