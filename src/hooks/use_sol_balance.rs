use dioxus::prelude::*;
use dioxus_std::utils::rw::use_rw;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use std::time::Duration;
#[cfg(feature = "web")]
use web_time::Duration;

use crate::gateway::{AsyncResult, GatewayError};

use super::{use_gateway, use_pubkey};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolBalance(pub u64);

#[derive(Clone)]
pub struct SolBalanceHandle(UseFuture<()>);

impl SolBalanceHandle {
    pub fn restart(&self) {
        self.0.restart();
    }
    pub fn cancel(&self, cx: &ScopeState) {
        self.0.cancel(cx);
    }
}

pub fn use_sol_balance(cx: &ScopeState) -> AsyncResult<SolBalance> {
    *use_shared_state::<AsyncResult<SolBalance>>(cx)
        .unwrap()
        .read()
}

pub fn use_sol_balance_provider(cx: &ScopeState) {
    use_shared_state_provider::<AsyncResult<SolBalance>>(cx, || AsyncResult::Loading);
    let balance_ = use_rw::<AsyncResult<SolBalance>>(cx, || AsyncResult::Loading);
    let balance = use_shared_state::<AsyncResult<SolBalance>>(cx).unwrap();
    let address = use_pubkey(cx);
    let gateway = use_gateway(cx);

    // Fetch initial balance.
    let f = use_future(cx, (), |_| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            // TODO Handle error
            let b = gateway.rpc.get_balance(&address).await.unwrap_or(0);
            *balance.write() = AsyncResult::Ok(SolBalance(b));
        }
    });

    // Poll for future balance changes
    let sub = use_future(cx, balance, |_| {
        let f = f.clone();
        let poll = 3;
        let b = *balance.read();
        async move {
            if let AsyncResult::Ok(b) = b {
                if b.0.eq(&0) {
                    loop {
                        async_std::task::sleep(Duration::from_secs(poll)).await;
                        f.restart();
                    }
                }
            }
        }
    });

    sub.cancel(cx);
    cx.provide_context(SolBalanceHandle(sub.clone()));

    // Write balance_ changes to shared state
    let balance__ = *balance_.read().unwrap();
    use_future(cx, &balance__, |_| {
        *balance.write() = balance__;
        async move {}
    });
}

pub fn _use_sol_account_balance(cx: &ScopeState, address: Pubkey) -> AsyncResult<SolBalance> {
    let balance = use_state::<AsyncResult<SolBalance>>(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);

    use_future(cx, (), |_| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            // TODO Handle error
            match gateway.rpc.get_balance(&address).await {
                Ok(b) => balance.set(AsyncResult::Ok(SolBalance(b))),
                Err(err) => balance.set(AsyncResult::Error(GatewayError::from(err))),
            }
        }
    });

    *balance.get()
}
