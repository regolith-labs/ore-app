use dioxus::prelude::*;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{wasm_client, AsyncResult};

pub fn use_ore_supply(cx: &ScopeState) -> (AsyncResult<UiTokenAmount>, &UseFuture<()>) {
    // TODO
    let client = wasm_client();
    let supply = use_state::<AsyncResult<UiTokenAmount>>(cx, || AsyncResult::Loading);

    let f = use_future(cx, (), |_| {
        let supply = supply.clone();
        async move {
            match client.get_token_supply(&ore::MINT_ADDRESS).await {
                Ok(token_amount) => {
                    supply.set(AsyncResult::Ok(token_amount));
                }
                Err(err) => {
                    supply.set(AsyncResult::Error(err.into()));
                }
            }
        }
    });

    (supply.get().clone(), f)
}
