use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;
use steel::Pubkey;

use super::utils::deserialize_pubkey;

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
