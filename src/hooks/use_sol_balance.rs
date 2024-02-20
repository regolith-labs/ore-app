use dioxus::prelude::*;
use dioxus_std::utils::rw::use_rw;

use crate::gateway::AsyncResult;

use super::{use_gateway, use_pubkey};

pub fn use_sol_balance(cx: &ScopeState) -> AsyncResult<u64> {
    // Balance state.
    let pubkey = use_pubkey(cx);
    let balance = use_rw::<AsyncResult<u64>>(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);

    // Fetch initial balance.
    let _ = use_future(cx, (), |_| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            // TODO Handle error
            let b = gateway.rpc.get_balance(&pubkey).await.unwrap_or(0);
            balance.write(AsyncResult::Ok(b)).unwrap();
        }
    });

    // Stream balance changes.
    let _: &Coroutine<()> = use_coroutine(cx, |mut _rx| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            let (sender, receiver) = async_std::channel::unbounded();
            let _ = gateway
                .rpc
                .account_subscribe(pubkey, move |data| {
                    async_std::task::block_on({
                        let sender = sender.clone();
                        async move {
                            let lamports = data.value.unwrap().lamports;
                            sender.send(lamports).await.unwrap();
                        }
                    });
                })
                .await;
            loop {
                if let Ok(lamports) = receiver.recv().await {
                    balance.write(AsyncResult::Ok(lamports)).unwrap();
                }
            }
        }
    });

    *balance.read().unwrap()
}
