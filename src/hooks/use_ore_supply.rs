use dioxus::prelude::*;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::AsyncResult;

use super::use_gateway;

pub fn use_ore_supply(cx: &ScopeState) -> (AsyncResult<UiTokenAmount>, &UseFuture<()>) {
    // TODO
    let gateway = use_gateway(cx);
    let supply = use_state::<AsyncResult<UiTokenAmount>>(cx, || AsyncResult::Loading);

    let f = use_future(cx, (), |_| {
        let supply = supply.clone();
        let gateway = gateway.clone();
        async move {
            match gateway.rpc.get_token_supply(&ore::MINT_ADDRESS).await {
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
