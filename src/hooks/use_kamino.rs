use dioxus::prelude::*;
use steel::Pubkey;

use crate::gateway::{kamino::{KaminoGateway, KaminoStrategyMetrics}, GatewayError, GatewayResult};

use super::use_gateway;

pub fn use_kamino_strategy_metrics(strategy: Pubkey) -> Resource<GatewayResult<KaminoStrategyMetrics>> {
    use_resource(move || async move {
        use_gateway().get_strategy_metrics(strategy).await.map_err(GatewayError::from)
    })
}
