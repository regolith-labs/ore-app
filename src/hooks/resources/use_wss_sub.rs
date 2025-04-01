use dioxus::prelude::*;
use futures::StreamExt;
use solana_sdk::pubkey::Pubkey;

use crate::gateway::{AccountNotificationParams, GatewayResult};

use super::{use_wss, FromWssMsg, ToWssMsg};

/// End to end management of websocket subscriptions.
///
/// We have exactly one wss connection and all notifications come thru the same channel.
/// This means we need to differentiate between subscribers, and route the correct notifs to
/// the respective subscribing components.
///
/// This hook manages
/// 1) Creating new subscriptions
/// 2) Routing notifications
/// 3) Closing subscriptions when the parent component unmounts
pub fn use_wss_subscription<T, U>(
    mut data: Signal<GatewayResult<T>>,
    update_callback: U,
) -> Coroutine<Pubkey>
where
    T: Clone + 'static,
    U: Fn(&AccountNotificationParams) -> GatewayResult<T> + 'static,
{
    let (from_wss, to_wss) = use_wss();
    let mut sub_id = use_signal(|| 0);
    let mut sub_request_id = use_signal(|| 0);

    // Subscribe when component mounts
    let pubkey_tx = use_coroutine(move |mut rx: UnboundedReceiver<Pubkey>| async move {
        while let Some(pubkey) = rx.next().await {
            let rid = fastrand::u64(..);
            // Set sub request id
            sub_request_id.set(rid);
            // Unsubscribe from previous wallet first
            let current_sub_id = *sub_id.read();
            if current_sub_id > 0 {
                to_wss.send(ToWssMsg::Unsubscribe(current_sub_id));
            }
            // Then subscribe to new wallet
            to_wss.send(ToWssMsg::Subscribe(rid, pubkey));
        }
    });

    // Handle subscription ID tracking
    use_effect(move || {
        let msg = from_wss.cloned();
        let sub_request_id = sub_request_id.cloned();
        // Track subscription ID
        if let FromWssMsg::Subscription(rid, sid) = msg {
            // Only handle subscriptions originating from this component
            if sub_request_id.eq(&rid) {
                sub_id.set(sid);
            }
        }
    });

    // Handle data updates
    use_effect(move || {
        let msg = from_wss.cloned();
        // Only process notification messages
        if let FromWssMsg::Notif(notif) = msg {
            if notif.subscription.eq(&sub_id()) {
                data.set(update_callback(&notif));
            }
        }
    });

    // Unsubscribe when component is dropped
    use_drop(move || {
        let current_sub_id = *sub_id.read();
        if current_sub_id > 0 {
            to_wss.send(ToWssMsg::Unsubscribe(current_sub_id));
        }
    });

    pubkey_tx
}
