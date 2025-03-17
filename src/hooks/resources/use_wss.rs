use dioxus::prelude::*;
use futures::{
    channel::mpsc::{self, Receiver, Sender},
    future::{Fuse, FusedFuture},
    pin_mut, FutureExt, SinkExt, StreamExt,
};
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

/// Two way channel backed by a WebSocket
/// for subscribing to notifications from the RPC server.
pub fn use_wss() -> (FromWss, ToWss) {
    let from = use_context::<Signal<FromWssMsg>>();
    let to = use_coroutine_handle::<ToWssMsg>();
    (from, to)
}

/// Impl
///
/// Two way channel backed by a WebSocket
/// for subscribing to notifications from the RPC server.
pub fn use_wss_provider() {
    // Init from wss
    let mut from_wss = use_context_provider(|| Signal::new(FromWssMsg::Init));
    // Init to wss
    let to_wss = use_coroutine(move |mut rx: UnboundedReceiver<ToWssMsg>| async move {
        if let Err(err) = async {
            // Create channel for sending commands to the WebSocket worker
            let (cmd_tx, cmd_rx) = mpsc::channel::<WssCommand>(10);

            // Clone signal for the WebSocket worker
            let mut from_wss_worker = from_wss.clone();

            // Spawn the WebSocket worker task that owns the WebSocket connection exclusively
            spawn(wss_worker(cmd_rx, from_wss_worker));

            // Handle UI commands and forward them to the WebSocket worker
            while let Some(msg) = rx.next().await {
                log::info!("to wss: {:?}", msg);
                match msg {
                    ToWssMsg::Subscribe(pubkey) => {
                        // Create a one-shot channel for the subscription ID response
                        let (sub_resp_tx, mut sub_resp_rx) = mpsc::channel::<SubId>(1);

                        // Send the subscribe command to the worker
                        if let Err(e) = cmd_tx
                            .clone()
                            .send(WssCommand::Subscribe(pubkey, sub_resp_tx))
                            .await
                        {
                            log::error!("Failed to send subscribe command: {:?}", e);
                            continue;
                        }

                        // Wait for the subscription ID response
                        if let Some(sub_id) = sub_resp_rx.next().await {
                            log::info!("sub id: {}", sub_id);
                            from_wss.set(FromWssMsg::Subscription(sub_id));
                        }
                    }
                    ToWssMsg::Unsubscribe(sub_id) => {
                        // Send the unsubscribe command to the worker
                        if let Err(e) = cmd_tx.clone().send(WssCommand::Unsubscribe(sub_id)).await {
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
    });
}

/// WebSocket worker function that owns the WebSocket connection exclusively
async fn wss_worker(mut cmd_rx: Receiver<WssCommand>, mut from_wss: Signal<FromWssMsg>) {
    // Connect to WebSocket
    let mut wss = match AccountSubscribeGateway::connect().await {
        Ok(wss) => wss,
        Err(e) => {
            log::error!("Failed to connect to WebSocket: {:?}", e);
            return;
        }
    };

    // Create a task for handling notifications
    let (notification_tx, mut notification_rx) = mpsc::channel(10);

    // Spawn a task to listen for notifications from the WebSocket
    let _notification_task = spawn(async move {
        while let Some(notif) = notification_rx.next().await {
            from_wss.set(FromWssMsg::Notif(notif));
        }
    });

    // Main loop to process commands and notifications
    loop {
        // Use select to handle both commands and WebSocket notifications concurrently
        futures::select! {
            // Handle commands from the UI
            cmd = cmd_rx.next() => {
                match cmd {
                    Some(WssCommand::Subscribe(pubkey, resp_tx)) => {
                        match wss.subscribe(pubkey.to_string().as_str()).await {
                            Ok(sub_id) => {
                            log::info!("here sub id: {}", sub_id);
                                if let Err(e) = resp_tx.clone().send(sub_id).await {
                                    log::error!("Failed to send subscription ID response: {:?}", e);
                                }
                            }
                            Err(err) => {
                                log::error!("Failed to subscribe: {:?}", err);
                            }
                        }
                    }
                    Some(WssCommand::Unsubscribe(sub_id)) => {
                        if let Err(err) = wss.unsubscribe(sub_id).await {
                            log::error!("Failed to unsubscribe: {:?}", err);
                        }
                    }
                    None => {
                        // Command channel closed, exit the worker
                        break;
                    }
                }
            }

            // Handle notifications from the WebSocket
            notification = wss.next_notification().fuse() => {
                match notification {
                    Ok(notification) => {
                        log::info!("WebSocket notification: {:?}", notification);
                        if let Err(e) = notification_tx.clone().send(notification.params).await {
                            log::error!("Failed to forward notification: {:?}", e);
                        }
                    }
                    Err(e) => {
                        log::error!("WebSocket notification error: {:?}", e);
                    }
                }
            }
        }
    }
}

// Internal message types for the WebSocket worker
#[derive(Debug)]
enum WssCommand {
    Subscribe(Pubkey, Sender<SubId>),
    Unsubscribe(SubId),
}
