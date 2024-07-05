use dioxus::prelude::*;
use ore_api::state::Treasury;

use crate::gateway::GatewayResult;

use super::use_gateway;

pub fn use_treasury() -> Resource<GatewayResult<Treasury>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move { gateway.get_treasury().await }
    })
}
