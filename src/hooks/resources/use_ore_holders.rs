use dioxus::prelude::*;

use crate::{gateway::{ore::OreGateway, GatewayError, GatewayResult}, hooks::use_gateway};

pub fn use_ore_holders() -> Resource<GatewayResult<u64>> {
    use_resource(move || async move {
        use_gateway().get_ore_holders().await.map_err(GatewayError::from)
    })
}