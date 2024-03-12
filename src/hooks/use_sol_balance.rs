use dioxus::prelude::*;
use dioxus_std::utils::rw::use_rw;
#[cfg(feature = "desktop")]
use solana_client::pubsub_client::PubsubClient;

use crate::gateway::AsyncResult;
#[cfg(feature = "desktop")]
use crate::gateway::RPC_WSS_URL;

use super::{use_gateway, use_pubkey};

// pub fn use_proof(cx: &ScopeState) -> &UseSharedState<AsyncResult<Proof>> {
//     use_shared_state::<AsyncResult<Proof>>(cx).unwrap()
// }

// pub fn use_proof_provider(cx: &ScopeState) {
//     use_shared_state_provider::<AsyncResult<Proof>>(cx, || AsyncResult::Loading);
//     let proof = use_shared_state::<AsyncResult<Proof>>(cx).unwrap();
//     let pubkey = use_pubkey(cx);
//     let proof_pubkey = proof_pubkey(pubkey);
//     let gateway = use_gateway(cx);

//     let f = use_future(cx, (), |_| {
//         let proof = proof.clone();
//         let gateway = gateway.clone();
//         async move {
//             if let Ok(data) = gateway.rpc.get_account_data(&proof_pubkey).await {
//                 if let Ok(p) = Proof::try_from_bytes(data.as_ref()) {
//                     *proof.write() = AsyncResult::Ok(*p);
//                 }
//             }
//         }
//     });

//     cx.provide_context(ProofHandle(f.clone()));
// }

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

    // Subscribe to balance changes
    use_future(cx, (), |_| {
        #[cfg(feature = "web")]
        let gateway = gateway.clone();
        let balance_ = balance_.clone();
        async move {
            #[cfg(feature = "web")]
            let _ = gateway
                .rpc
                .account_subscribe(address, move |account| {
                    let lamports = account.value.unwrap().lamports;
                    balance_.write(AsyncResult::Ok(SolBalance(lamports))).ok();
                })
                .await;

            #[cfg(feature = "desktop")]
            std::thread::spawn(move || {
                match PubsubClient::account_subscribe(
                    RPC_WSS_URL,
                    &address,
                    Some(solana_client::rpc_config::RpcAccountInfoConfig::default()),
                ) {
                    Ok((mut _sub, rx)) => {
                        while let Ok(ui_account) = rx.recv() {
                            let lamports = ui_account.value.lamports;
                            balance_.write(AsyncResult::Ok(lamports)).unwrap();
                        }
                    }
                    Err(err) => {
                        log::error!("Failed to subscribe to account: {:?}", err)
                    }
                };
            });
        }
    });

    // Write balance_ changes to shared state
    let balance__ = *balance_.read().unwrap();
    use_future(cx, &balance__, |_| {
        *balance.write() = balance__;
        async move {}
    });
}

// pub fn use_sol_balance(cx: &ScopeState) -> AsyncResult<u64> {
//     // Balance state.
//     let address = use_pubkey(cx);
//     let balance = use_rw::<AsyncResult<u64>>(cx, || AsyncResult::Loading);
//     let gateway = use_gateway(cx);

//     // Fetch initial balance.
//     let _ = use_future(cx, (), |_| {
//         let balance = balance.clone();
//         let gateway = gateway.clone();
//         async move {
//             // TODO Handle error
//             let b = gateway.rpc.get_balance(&address).await.unwrap_or(0);
//             balance.write(AsyncResult::Ok(b)).unwrap();
//         }
//     });

//     // TODO Wasm account subscribe does not support more than two accounts at a time
//     // Stream balance changes.
//     // let _: &Coroutine<()> = use_coroutine(cx, |mut _rx| {
//     //     #[cfg(feature = "web")]
//     //     let gateway = gateway.clone();
//     //     let balance = balance.clone();
//     //     async move {
//     //         #[cfg(feature = "web")]
//     //         let _ = gateway
//     //             .rpc
//     //             .account_subscribe(address, move |account| {
//     //                 let lamports = account.value.unwrap().lamports;
//     //                 balance.write(AsyncResult::Ok(lamports)).unwrap();
//     //             })
//     //             .await;

//     //         #[cfg(feature = "desktop")]
//     //         std::thread::spawn(move || {
//     //             match PubsubClient::account_subscribe(
//     //                 RPC_WSS_URL,
//     //                 &address,
//     //                 Some(solana_client::rpc_config::RpcAccountInfoConfig::default()),
//     //             ) {
//     //                 Ok((mut _sub, rx)) => {
//     //                     while let Ok(ui_account) = rx.recv() {
//     //                         let lamports = ui_account.value.lamports;
//     //                         balance.write(AsyncResult::Ok(lamports)).unwrap();
//     //                     }
//     //                 }
//     //                 Err(err) => {
//     //                     log::error!("Failed to subscribe to account: {:?}", err)
//     //                 }
//     //             };
//     //         });
//     //     }
//     // });

//     *balance.read().unwrap()
// }
