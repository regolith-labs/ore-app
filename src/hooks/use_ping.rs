use dioxus::prelude::*;

use crate::gateway::{wasm_client, AsyncResult};

pub fn use_ping(cx: &ScopeState) -> AsyncResult<u64> {
    let ping_status = use_state(cx, || AsyncResult::Loading);

    use_future(cx, (), |_| {
        let ping_status = ping_status.clone();
        async move {
            let client = wasm_client();
            loop {
                match client.get_slot().await {
                    Ok(slot) => ping_status.set(AsyncResult::Ok(slot)),
                    Err(err) => ping_status.set(AsyncResult::Error(err.into())),
                }
                async_std::task::sleep(std::time::Duration::from_secs(60)).await;
            }
        }
    });

    *ping_status.get()
}
