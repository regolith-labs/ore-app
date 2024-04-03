use dioxus::prelude::*;

use crate::gateway::AsyncResult;

use super::use_gateway;

pub fn use_ping(cx: &ScopeState) -> AsyncResult<u64> {
    let gateway = use_gateway(cx);
    let ping_status = use_state(cx, || AsyncResult::Loading);

    use_future(cx, (), |_| {
        let ping_status = ping_status.clone();
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

    *ping_status.get()
}
