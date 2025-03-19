use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::gateway::{
    AccountNotificationParams, AccountSubscribe, AccountSubscribeGateway, GatewayResult,
};

use super::{use_wss, FromWssMsg, ToWssMsg};

pub fn use_wss_subscription<T, U>(
    mut data: Signal<GatewayResult<T>>,
    update_callback: U,
    pubkey: Pubkey,
) where
    T: Clone + 'static,
    U: Fn(&AccountNotificationParams) -> GatewayResult<T> + 'static,
{
    let (from_wss, to_wss) = use_wss();
    let mut sub_id = use_signal(|| 0);
    let sub_request_id = use_memo(move || AccountSubscribeGateway::request_id());

    // Handle subscription ID tracking
    use_effect(move || {
        let msg = from_wss.cloned();
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

    // Subscribe when component mounts
    use_effect(move || {
        to_wss.send(ToWssMsg::Subscribe(sub_request_id(), pubkey));
    });

    // Unsubscribe when component is dropped
    use_drop(move || {
        let current_sub_id = *sub_id.read();
        if current_sub_id > 0 {
            to_wss.send(ToWssMsg::Unsubscribe(current_sub_id));
        }
    });
}
