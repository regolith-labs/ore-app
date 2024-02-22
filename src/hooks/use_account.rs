use dioxus::prelude::*;
use dioxus_std::utils::rw::{use_rw, UseRw};
use ore::utils::AccountDeserialize;
use solana_client_wasm::solana_sdk::{account::Account, pubkey::Pubkey};

use crate::gateway::AsyncResult;

use super::use_gateway;

pub fn use_account<
    T: AccountDeserialize + Send + Sync + Clone + Copy + std::fmt::Debug + 'static,
>(
    cx: &ScopeState,
    address: Pubkey,
) -> (&mut UseRw<AsyncResult<T>>, &UseFuture<()>) {
    let acc = use_rw::<AsyncResult<T>>(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);

    let f = use_future(cx, (), |_| {
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

    (acc, f)
}

// NOTE: For some reason, the rpc subscription only works with one account globally.
// Even if a second rpc client is used for the second subscription, generating different subscription ids,
// one of them will inevitably throw "dropped closure" errors. It's not clear why this is.
// So for now, this hook can only be used once globally throughout the app.
// We'll use it to subscribe to the treasury account in the miner toolbar and manually refresh all other accounts.
pub fn use_account_subscribe<
    'a,
    T: AccountDeserialize + Send + Sync + Clone + Copy + std::fmt::Debug + 'static,
>(
    cx: &'a ScopeState,
    address: Pubkey,
    rw: &'a mut UseRw<AsyncResult<T>>,
) -> &'a Coroutine<()> {
    let gateway = use_gateway(cx);
    use_coroutine(cx, |mut _rx| {
        let gateway = gateway.clone();
        let rw = rw.clone();
        async move {
            let _ = gateway
                .rpc
                .account_subscribe(address, move |account| {
                    if let Some(ui_account) = account.value {
                        if let Some(account) = ui_account.decode::<Account>() {
                            if let Ok(t) = T::try_from_bytes(account.data.as_ref()) {
                                rw.write(AsyncResult::Ok(*t)).ok();
                            }
                        }
                    }
                })
                .await;
        }
    })
}
