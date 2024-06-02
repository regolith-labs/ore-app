use dioxus::prelude::*;

use crate::gateway::AsyncResult;

use super::use_gateway;

pub fn use_ping() -> Signal<AsyncResult<u64>> {
    let gateway = use_gateway();
    let mut ping_status = use_signal(|| AsyncResult::Loading);

    use_future(move || {
        let gateway = gateway.clone();
        async move {
            loop {
                match gateway.rpc.get_slot().await {
                    Ok(slot) => ping_status.set(AsyncResult::Ok(slot)),
                    Err(err) => ping_status.set(AsyncResult::Error(err.into())),
                }
                async_std::task::sleep(std::time::Duration::from_secs(180)).await;
            }
        }
    });

    ping_status
}
