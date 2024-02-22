use dioxus::prelude::*;
use dioxus_std::utils::rw::{use_rw, UseRw};
use ore::utils::AccountDeserialize;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

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
