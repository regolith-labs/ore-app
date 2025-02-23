use once_cell::sync::Lazy;
use serde::Deserialize;
use steel::Pubkey;

use crate::utils::deserialize_pubkey;

pub static LISTED_POOLS: Lazy<Vec<Pool>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-pools.yaml");

    // Parse the config
    let config: Config = serde_yaml::from_str(yaml_str).expect("Failed to parse listed-pools.yaml");

    config.pools
});

pub const FIRST_POOL: Lazy<Pool> = Lazy::new(|| {
    LISTED_POOLS
        .first()
        .expect("Must be at least one entry in listed-pools.yaml")
        .clone()
});

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Pool {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub address: Pubkey,
    pub url: String,
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Deserialize)]
struct Config {
    pools: Vec<Pool>,
}
