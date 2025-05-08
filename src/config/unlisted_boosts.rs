use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;
use steel::Pubkey;

use crate::utils::{deserialize_pubkey, deserialize_pubkey_option};

// Create a static HashMap indexed by ticker
pub static UNLISTED_BOOSTS: Lazy<Vec<UnlistedBoostMeta>> = Lazy::new(|| {
    let yaml_str = include_str!("../../public/config/unlisted-boosts.yaml");
    let config: UnlistedConfig =
        serde_yaml::from_str(yaml_str).expect("Failed to parse unlisted-boosts.yaml");
    config.boosts
});

pub static UNLISTED_BOOSTS_BY_MINT: Lazy<HashMap<Pubkey, UnlistedBoostMeta>> = Lazy::new(|| {
    let yaml_str = include_str!("../../public/config/unlisted-boosts.yaml");
    log::info!("yaml_str: {}", yaml_str);
    let config: UnlistedConfig =
        serde_yaml::from_str(yaml_str).expect("Failed to parse unlisted-boosts.yaml");
    config
        .boosts
        .into_iter()
        .map(|boost| (boost.mint, boost))
        .collect()
});

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct UnlistedBoostMeta {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub address: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub mint: Pubkey,
    pub name: String,
    pub ticker: String,
}

#[derive(Deserialize)]
struct UnlistedConfig {
    boosts: Vec<UnlistedBoostMeta>,
}
