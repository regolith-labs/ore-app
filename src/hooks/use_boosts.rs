use std::{collections::HashMap, str::FromStr};

use dioxus::prelude::*;
use once_cell::sync::Lazy;
use ore_boost_api::state::Boost;
use serde::{Deserialize, Deserializer};
use steel::Pubkey;

use crate::gateway::{ore::OreGateway, GatewayError, GatewayResult};
use super::use_gateway;

// Create a static HashMap indexed by ticker
pub static LISTED_BOOSTS: Lazy<HashMap<String, BoostMeta>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-boosts.yaml");

    // Parse the config
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-boosts.yaml");

    // Convert Vec<Asset> into HashMap<String, Asset>
    config
        .boosts
        .into_iter()
        .map(|boost| (boost.ticker.clone(), boost))
        .collect()
});

pub fn use_boosts() -> Resource<GatewayResult<Vec<Boost>>> {
    use_resource(move || async move {
        use_gateway().rpc.get_boosts().await.map_err(GatewayError::from)
    })
}

pub fn use_boost(mint: Pubkey) -> Resource<GatewayResult<Boost>> {
    use_resource(move || async move {
        use_gateway().rpc.get_boost(mint).await.map_err(GatewayError::from)
    })
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct BoostMeta {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub address: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub lp_mint: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub pair_mint: Pubkey,
    pub name: String,
    pub ticker: String,
    pub link: String,
}


#[derive(Deserialize)]
struct Config {
    boosts: Vec<BoostMeta>,
}

fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}

impl BoostMeta {}
