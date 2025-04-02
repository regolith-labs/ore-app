use dioxus::prelude::*;

use dioxus::prelude::*;
use futures::{
    channel::mpsc::{self, Receiver, Sender},
    FutureExt, SinkExt, StreamExt,
};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

use crate::gateway::{
    AccountNotificationParams, AccountSubscribe, AccountSubscribeGateway, GatewayError,
};
use crate::time::sleep;

pub type FromWss = Signal<FromWssMsg>;
pub type ToWss = Coroutine<ToWssMsg>;
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FromWssMsg {
    Init,
    Subscription(SubRequestId, SubId),
    Notif(AccountNotificationParams),
}
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ToWssMsg {
    Subscribe(SubRequestId, Pubkey),
    Unsubscribe(SubId),
}
type SubId = u64;
type SubRequestId = u64;

/// Two way channel backed by a WebSocket
/// for subscribing to notifications from the RPC server.
pub(super) fn use_wss() -> (FromWss, ToWss) {
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
    let _to_wss = use_coroutine(move |mut rx: UnboundedReceiver<ToWssMsg>| async move {
        if let Err(err) = async {
            // Create channel for sending commands to the WebSocket worker
            let (cmd_tx, cmd_rx) = mpsc::channel::<WssCommand>(100);

            // Spawn the WebSocket worker task that owns the WebSocket connection exclusively
            spawn(wss_worker(cmd_rx, from_wss.clone()));

            // Handle UI commands and forward them to the WebSocket worker
            while let Some(msg) = rx.next().await {
                match msg {
                    ToWssMsg::Subscribe(request_id, pubkey) => {
                        // Create a one-shot channel for the subscription ID response
                        let (sub_resp_tx, mut sub_resp_rx) = mpsc::channel::<SubId>(1);

                        // Send the subscribe command to the worker
                        if let Err(e) = cmd_tx
                            .clone()
                            .send(WssCommand::Subscribe(request_id, pubkey, sub_resp_tx))
                            .await
                        {
                            log::error!("Failed to send subscribe command: {:?}", e);
                            continue;
                        }

                        // Wait for the subscription ID response
                        if let Some(sub_id) = sub_resp_rx.next().await {
                            from_wss.set(FromWssMsg::Subscription(request_id, sub_id));
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
    // Map SubId -> (Pubkey, SubRequestId)
    let mut active_subscriptions: HashMap<SubId, (Pubkey, SubRequestId)> = HashMap::new();
    let mut retry_delay_ms = 1000u64; // Start with 1 second
    const MAX_RETRY_DELAY_MS: u64 = 60 * 1000; // 60 seconds

    // Outer loop for handling reconnections
    'reconnect: loop {
        // Attempt to connect
        log::info!("Attempting WebSocket connection...");
        let mut wss = match AccountSubscribeGateway::connect().await {
            Ok(wss) => {
                log::info!("WebSocket connected successfully.");
                retry_delay_ms = 1000; // Reset delay on successful connection
                wss
            }
            Err(e) => {
                log::error!(
                    "Failed to connect to WebSocket: {:?}. Retrying in {}ms...",
                    e,
                    retry_delay_ms
                );
                sleep(retry_delay_ms).await;
                retry_delay_ms = (retry_delay_ms * 2).min(MAX_RETRY_DELAY_MS);
                continue 'reconnect; // Retry connection
            }
        };

        // Re-subscribe to all previously active subscriptions
        let subscriptions_to_restore = active_subscriptions.values().cloned().collect::<Vec<_>>();
        active_subscriptions.clear(); // Clear old IDs, will be repopulated with new ones

        log::info!(
            "Attempting to restore {} subscriptions...",
            subscriptions_to_restore.len()
        );
        for (pubkey, request_id) in subscriptions_to_restore {
            match wss.subscribe(pubkey.to_string().as_str(), request_id).await {
                Ok(new_sub_id) => {
                    log::info!(
                        "Successfully re-subscribed pubkey {} (request_id: {}): new sub_id {}",
                        pubkey,
                        request_id,
                        new_sub_id
                    );
                    active_subscriptions.insert(new_sub_id, (pubkey, request_id));
                    // Notify UI about the new subscription ID for the original request
                    from_wss.set(FromWssMsg::Subscription(request_id, new_sub_id));
                }
                Err(err) => {
                    log::error!(
                        "Failed to re-subscribe pubkey {} (request_id: {}): {:?}",
                        pubkey,
                        request_id,
                        err
                    );
                    // TODO: How to signal this failure to the specific subscriber?
                    // For now, the subscription is simply lost. The UI component might
                    // need to detect the lack of updates and potentially retry manually.
                }
            }
        }
        log::info!("Finished attempting subscription restoration.");

        // Create a channel for forwarding notifications to the handler task
        // This needs to be recreated on each connection attempt as the old task might have ended
        let (notification_tx, mut notification_rx) = mpsc::channel(10);

        // Spawn a task to listen for notifications forwarded from the select loop
        // This task reads from notification_rx and updates the UI state (from_wss)
        let _notification_handler_task = spawn(async move {
            while let Some(notif) = notification_rx.next().await {
                from_wss.set(FromWssMsg::Notif(notif));
            }
            log::warn!("Notification handler task finished."); // Should ideally not happen unless channel closes
        });

        // Inner loop to process commands and notifications for the current connection
        loop {
            futures::select! {
                // Handle commands from the UI coroutine
                cmd = cmd_rx.next() => {
                    match cmd {
                        Some(WssCommand::Subscribe(request_id, pubkey, resp_tx)) => {
                            // TODO: Need to handle potential errors during subscribe/unsubscribe
                            // that might indicate a dead connection, potentially triggering reconnect.
                            match wss.subscribe(pubkey.to_string().as_str(), request_id).await {
                                Ok(sub_id) => {
                                    log::info!(
                                        "Subscribed pubkey {} (request_id: {}): sub_id {}",
                                        pubkey,
                                        request_id,
                                        sub_id
                                    );
                                    // Track the active subscription
                                    active_subscriptions.insert(sub_id, (pubkey, request_id));
                                    // Send subscription ID back to the caller (use_wss_subscription)
                                    if resp_tx.clone().send(sub_id).await.is_err() {
                                        log::warn!("Failed to send subscription ID response back to caller (channel closed).");
                                        // If we can't send back, the caller might be gone. Clean up?
                                        // Attempt to unsubscribe immediately if the caller is gone.
                                        if let Err(e) = wss.unsubscribe(sub_id).await {
                                             log::error!("Failed to auto-unsubscribe after caller disappeared: {:?}", e);
                                        }
                                        active_subscriptions.remove(&sub_id);
                                    }
                                    // Also notify the general listener (redundant with resp_tx but follows pattern)
                                    // This ensures the UI gets the initial sub_id via the FromWssMsg stream too.
                                    from_wss.set(FromWssMsg::Subscription(request_id, sub_id));
                                }
                                Err(err) => {
                                    log::error!("Failed to subscribe pubkey {} (request_id: {}): {:?}", pubkey, request_id, err);
                                    // TODO: Should we signal failure back via resp_tx? Requires changing resp_tx type.
                                    // Consider if this error warrants a reconnect attempt
                                }
                            }
                        }
                        Some(WssCommand::Unsubscribe(sub_id)) => {
                            // Remove from tracking *before* sending unsubscribe, in case of error
                            if let Some((pubkey, request_id)) = active_subscriptions.remove(&sub_id) {
                                log::info!(
                                    "Unsubscribing sub_id {} (pubkey: {}, request_id: {})",
                                    sub_id, pubkey, request_id
                                );
                                if let Err(err) = wss.unsubscribe(sub_id).await {
                                    log::error!("Failed to unsubscribe sub_id {}: {:?}", sub_id, err);
                                    // If unsubscribe fails, should we put it back in the map?
                                    // Probably not, as the state is now uncertain.
                                    // Consider if this error warrants a reconnect attempt.
                                }
                            } else {
                                log::warn!("Attempted to unsubscribe unknown sub_id: {}", sub_id);
                            }
                        }
                        None => {
                            // Command channel closed, UI coroutine likely dropped. Exit the worker completely.
                            log::info!("Command channel closed. Exiting WebSocket worker.");
                            return; // Exit the entire wss_worker function
                        }
                    }
                }

                // Handle notifications from the WebSocket connection
                notification_result = wss.next_notification().fuse() => {
                    match notification_result {
                        Ok(notification) => {
                            // Forward the notification to the handler task via the channel
                            if let Err(e) = notification_tx.clone().send(notification.params).await {
                                log::error!("Failed to forward notification to handler task: {:?}. Channel likely closed.", e);
                                // If the channel is closed, the handler task might have panicked or finished.
                                // This might indicate a need to restart the connection/worker.
                                // For now, we'll log and continue, but this could be a reconnect trigger.
                            }
                        }
                        Err(e) => {
                            // An error here likely means the WebSocket connection is broken.
                            log::error!("WebSocket notification error: {:?}. Triggering reconnect.", e);
                            // Break the inner loop to trigger reconnection in the outer loop
                            break; // Exit inner loop, go to 'reconnect loop start
                        }
                    }
                }

                // Make select! biased towards completion to avoid starvation if one branch is always ready
                complete => break, // Exit inner loop if select! completes (e.g., both futures resolved/closed)
            }
        }

        // If we break out of the inner loop due to an error, prepare for reconnection attempt
        log::warn!(
            "WebSocket connection lost or error occurred. Attempting reconnect after {}ms delay...",
            retry_delay_ms
        );
        sleep(retry_delay_ms).await;
        retry_delay_ms = (retry_delay_ms * 2).min(MAX_RETRY_DELAY_MS);
        // The outer 'reconnect loop will now iterate, attempting to connect again.
    }
}

// Internal message types for the WebSocket worker
#[derive(Debug)]
enum WssCommand {
    Subscribe(SubRequestId, Pubkey, Sender<SubId>),
    Unsubscribe(SubId),
}
