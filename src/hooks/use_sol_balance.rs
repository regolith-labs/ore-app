use dioxus::prelude::*;
use dioxus_std::utils::rw::use_rw;
#[cfg(feature = "desktop")]
use solana_client::pubsub_client::PubsubClient;

use crate::gateway::AsyncResult;
#[cfg(feature = "desktop")]
use crate::gateway::RPC_WSS_URL;

use super::{use_gateway, use_pubkey};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolBalance(pub u64);

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
    use_future(cx, (), |_| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            // TODO Handle error
            let b = gateway.rpc.get_balance(&address).await.unwrap_or(0);
            *balance.write() = AsyncResult::Ok(SolBalance(b));
        }
    });

    // // Subscribe to balance changes
    // use_future(cx, (), |_| {
    //     #[cfg(feature = "web")]
    //     let gateway = gateway.clone();
    //     let balance_ = balance_.clone();
    //     async move {
    //         #[cfg(feature = "web")]
    //         let _ = gateway
    //             .rpc
    //             .account_subscribe(address, move |account| {
    //                 let lamports = account.value.unwrap().lamports;
    //                 balance_.write(AsyncResult::Ok(SolBalance(lamports))).ok();
    //             })
    //             .await;

    //         #[cfg(feature = "desktop")]
    //         std::thread::spawn(move || {
    //             match PubsubClient::account_subscribe(
    //                 RPC_WSS_URL,
    //                 &address,
    //                 Some(solana_client::rpc_config::RpcAccountInfoConfig::default()),
    //             ) {
    //                 Ok((mut _sub, rx)) => {
    //                     while let Ok(ui_account) = rx.recv() {
    //                         let lamports = ui_account.value.lamports;
    //                         balance_
    //                             .write(AsyncResult::Ok(SolBalance(lamports)))
    //                             .unwrap();
    //                     }
    //                 }
    //                 Err(err) => {
    //                     log::error!("Failed to subscribe to account: {:?}", err)
    //                 }
    //             };
    //         });
    //     }
    // });

    // Write balance_ changes to shared state
    let balance__ = *balance_.read().unwrap();
    use_future(cx, &balance__, |_| {
        *balance.write() = balance__;
        async move {}
    });
}
