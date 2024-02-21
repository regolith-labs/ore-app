use dioxus::prelude::*;
use dioxus_std::utils::rw::use_rw;
use ore::utils::AccountDeserialize;
use solana_client_wasm::solana_sdk::{account::Account, pubkey::Pubkey};

use crate::gateway::AsyncResult;

use super::use_gateway;

pub fn use_account<T: AccountDeserialize + Send + Sync + Clone + Copy + 'static>(
    cx: &ScopeState,
    address: Pubkey,
) -> AsyncResult<T> {
    let acc = use_rw::<AsyncResult<T>>(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);

    // Fetch account.
    let _ = use_future(cx, (), |_| {
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

    // Stream changes.
    // let _: &Coroutine<()> = use_coroutine(cx, |mut _rx| {
    //     let acc = acc.clone();
    //     let gateway = gateway.clone();
    //     async move {
    //         // let (sender, receiver) = async_std::channel::unbounded();
    //         let _ = gateway
    //             .rpc
    //             .account_subscribe(address, move |account| {
    //                 // async_std::task::block_on({
    //                 // let sender = sender.clone();
    //                 // async_std::task::spawn({
    //                 let acc = acc.clone();
    //                 wasm_bindgen_futures::spawn_local(async move {
    //                     if let Some(account) = account.value.unwrap().decode::<Account>() {
    //                         if let Ok(t) = T::try_from_bytes(account.data.as_ref()) {
    //                             acc.write(AsyncResult::Ok(*t)).unwrap();
    //                             // sender.send(*t).await.unwrap();
    //                         }
    //                     }
    //                 });
    //                 // });
    //                 // });
    //             })
    //             .await;
    //         // loop {
    //         //     if let Ok(result) = receiver.recv().await {
    //         //         acc.write(AsyncResult::Ok(result)).unwrap();
    //         //     }
    //         // }
    //     }
    // });

    *acc.read().unwrap()
}
