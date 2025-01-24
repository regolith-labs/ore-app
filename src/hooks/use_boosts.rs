use dioxus::prelude::*;
use ore_boost_api::state::Boost;

use crate::gateway::{ore::OreGateway, GatewayError, GatewayResult};

use super::use_gateway;

pub fn use_boosts() -> Resource<GatewayResult<Vec<Boost>>> {
    use_resource(move || async move {
        use_gateway().rpc.get_boosts().await.map_err(GatewayError::from)
    })
}
