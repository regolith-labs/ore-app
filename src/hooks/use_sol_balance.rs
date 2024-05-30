use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use web_time::Duration;

use crate::gateway::{AsyncResult, GatewayError};

use super::{use_gateway, use_pubkey};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolBalance(pub u64);

#[derive(Clone)]
pub struct SolBalanceHandle(UseFuture);

impl SolBalanceHandle {
    pub fn restart(&mut self) {
        self.0.restart();
    }
    pub fn cancel(&mut self) {
        self.0.cancel();
    }
}

pub fn use_sol_balance() -> Signal<AsyncResult<SolBalance>> {
    use_context::<Signal<AsyncResult<SolBalance>>>()
}

pub fn use_sol_balance_provider() {
    use_context_provider::<Signal<AsyncResult<SolBalance>>>(|| Signal::new(AsyncResult::Loading));
    let mut balance = use_context::<Signal<AsyncResult<SolBalance>>>();
    let address = use_pubkey();
    let gateway = use_gateway();

    // Fetch initial balance.
    let f = use_future(move || {
        let mut balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            // TODO Handle error
            let b = gateway.rpc.get_balance(&address).await.unwrap_or(0);
            balance.set(AsyncResult::Ok(SolBalance(b)));
        }
    });

    // Poll for future balance changes
    let mut sub = use_future(move || {
        let mut f = f.clone();
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

    sub.cancel();
    use_context_provider(|| Signal::new(SolBalanceHandle(sub.clone())));
}

pub fn _use_sol_account_balance(address: Pubkey) -> Signal<AsyncResult<SolBalance>> {
    let mut balance = use_signal::<AsyncResult<SolBalance>>(|| AsyncResult::Loading);
    let gateway = use_gateway();

    use_future(move || {
        let gateway = gateway.clone();
        async move {
            // TODO Handle error
            match gateway.rpc.get_balance(&address).await {
                Ok(b) => balance.set(AsyncResult::Ok(SolBalance(b))),
                Err(err) => balance.set(AsyncResult::Error(GatewayError::from(err))),
            }
        }
    });

    balance
}
