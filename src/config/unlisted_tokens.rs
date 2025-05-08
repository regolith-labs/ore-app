use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use super::Token;

// Create a static HashMap indexed by ticker
pub static UNLISTED_TOKENS_BY_TICKER: Lazy<HashMap<String, Token>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/unlisted-tokens.yaml");

    // Parse the config
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse unlisted-tokens.yaml");

    // Convert Vec<Asset> into HashMap<String, Asset>
    config
        .tokens
        .into_iter()
        .map(|asset| (asset.ticker.clone(), asset))
        .collect()
});

// Create a static HashMap indexed by ticker
pub static UNLISTED_TOKENS: Lazy<HashMap<Pubkey, Token>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/unlisted-tokens.yaml");

    // Parse the config
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse unlisted-tokens.yaml");

    // Convert Vec<Asset> into HashMap<Pubkey, Asset>
    config
        .tokens
        .into_iter()
        .map(|asset| (asset.mint, asset))
        .collect()
});

#[derive(Deserialize)]
struct Config {
    tokens: Vec<Token>,
}
