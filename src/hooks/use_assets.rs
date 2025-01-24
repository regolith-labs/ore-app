use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use solana_sdk::pubkey::Pubkey;

// Create a static HashMap indexed by ticker
pub static ASSETS: Lazy<HashMap<String, Asset>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-tokens.yaml");

    // Parse the config
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-tokens.yaml");

    // Convert Vec<Asset> into HashMap<String, Asset>
    config
        .assets
        .into_iter()
        .map(|asset| (asset.ticker.clone(), asset))
        .collect()
});

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Asset {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub mint: Pubkey,
    pub name: String,
    pub ticker: String,
    pub description: String,
    pub image: String,
    pub twitter: String,
    pub homepage: String,
    pub decimals: u8,
}

#[derive(Deserialize)]
struct Config {
    assets: Vec<Asset>,
}

fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}

impl Asset {
    pub fn ore() -> Self {
        ASSETS.get("ORE").cloned().unwrap()
    }

    pub fn sol() -> Self {
        ASSETS.get("SOL").cloned().unwrap()
    }
}
