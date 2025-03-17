// use dioxus::prelude::*;
//
// fn use_account_subscription(account: &str) -> Signal<Option<AccountData>> {
//     let signal = use_signal(|| None);
//     let subscription_id = use_ref(|| None::<u64>);
//     let client = use_ref(|| None::<AccountSubscriptionClient>);
//
//     use_effect(move || {
//         let account = account.to_string();
//
//         spawn(async move {
//             // Connect and subscribe
//             let mut client_instance = match AccountSubscriptionClient::connect().await {
//                 Ok(c) => c,
//                 Err(_) => return,
//             };
//
//             let sub_id = match client_instance.subscribe(&account).await {
//                 Ok(id) => id,
//                 Err(_) => return,
//             };
//
//             // Store the client and subscription ID
//             *client.write() = Some(client_instance);
//             *subscription_id.write() = Some(sub_id);
//
//             // Start listening for notifications
//             // This would depend on your specific implementation
//         });
//
//         // Cleanup function that runs when the component unmounts
//         move || {
//             if let (Some(client_instance), Some(sub_id)) =
//                 (client.read().clone(), *subscription_id.read())
//             {
//                 spawn(async move {
//                     let _ = client_instance.unsubscribe(sub_id).await;
//                 });
//             }
//         }
//     });
//
//     signal
// }
