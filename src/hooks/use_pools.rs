use std::str::FromStr;

use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

use crate::steel_app::solana::sdk::pubkey::Pubkey;


pub static POOLS: Lazy<Vec<Pool>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-pools.yaml");
    
    // Parse the config
    let config: PoolConfig = serde_yaml::from_str(yaml_str)
        .expect("Failed to parse listed-pools.yaml");

    config.pools
});

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Pool {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub address: Pubkey,
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Deserialize)]
struct PoolConfig {
    pools: Vec<Pool>,
}

fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
} 