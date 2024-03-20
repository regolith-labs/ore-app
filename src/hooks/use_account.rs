#[cfg(feature = "desktop")]
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_std::utils::rw::{use_rw, UseRw};
use ore::utils::AccountDeserialize;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;
#[cfg(feature = "web")]
use web_time::Duration;

use crate::gateway::AsyncResult;
#[cfg(feature = "desktop")]
use crate::gateway::RPC_WSS_URL;

use super::use_gateway;

pub fn use_account<
    T: AccountDeserialize + Send + Sync + Clone + Copy + std::fmt::Debug + 'static,
>(
    cx: &ScopeState,
    address: Pubkey,
    poll: Option<u64>,
) -> (&mut UseRw<AsyncResult<T>>, &UseFuture<()>) {
    let acc = use_rw::<AsyncResult<T>>(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);

    let f = use_future(cx, (), |_| {
        log::info!("Refetching account: {:?}", address);
        let acc = acc.clone();
        let gateway = gateway.clone();
        async move {
            if let Ok(data) = gateway.rpc.get_account_data(&address).await {
                if let Ok(t) = T::try_from_bytes(data.as_ref()) {
                    acc.write(AsyncResult::Ok(*t)).unwrap();
                }
            }
        }
    });

    use_future(cx, (), |_| {
        let f = f.clone();
        async move {
            if let Some(d) = poll {
                async_std::task::sleep(Duration::from_secs(d)).await;
                f.restart();
            }
        }
    });

    (acc, f)
}

// NOTE: For some reason, the rpc subscription only works with one account globally.
// Even if a second rpc client is used for the second subscription, generating different subscription ids,
// one of them will inevitably throw "dropped closure" errors. It's not clear why this is.
// So for now, this hook can only be used once globally throughout the app.
// We'll use it to subscribe to the treasury account in the miner toolbar and manually refresh all other accounts.
// pub fn use_account_subscribe<
//     'a,
//     T: AccountDeserialize + Send + Sync + Clone + Copy + std::fmt::Debug + 'static,
// >(
//     cx: &'a ScopeState,
//     address: Pubkey,
//     rw: &'a mut UseRw<AsyncResult<T>>,
// ) -> &'a Coroutine<()> {
//     #[cfg(feature = "web")]
//     let gateway = use_gateway(cx);

//     use_coroutine(cx, |mut _rx| {
//         #[cfg(feature = "web")]
//         let gateway = gateway.clone();
//         let rw = rw.clone();
//         async move {
//             #[cfg(feature = "web")]
//             let _ = gateway
//                 .rpc
//                 .account_subscribe(address, move |account| {
//                     if let Some(ui_account) = account.value {
//                         if let Some(account) = ui_account.decode::<Account>() {
//                             if let Ok(t) = T::try_from_bytes(account.data.as_ref()) {
//                                 rw.write(AsyncResult::Ok(*t)).ok();
//                             }
//                         }
//                     }
//                 })
//                 .await;

//             #[cfg(feature = "desktop")]
//             std::thread::spawn(move || {
//                 match PubsubClient::account_subscribe(
//                     RPC_WSS_URL,
//                     &address,
//                     Some(solana_client::rpc_config::RpcAccountInfoConfig::default()),
//                 ) {
//                     Ok((mut _sub, rx)) => {
//                         while let Ok(message) = rx.recv() {
//                             if let UiAccountData::LegacyBinary(data) = message.value.data {
//                                 if let Ok(t) = T::try_from_bytes(data.into_bytes().as_ref()) {
//                                     rw.write(AsyncResult::Ok(*t)).ok();
//                                 }
//                             }
//                         }
//                     }
//                     Err(err) => {
//                         log::error!("Failed to subscribe to account: {:?}", err)
//                     }
//                 };
//             });
//         }
//     })
// }
