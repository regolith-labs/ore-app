use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use crate::utils::deserialize_pubkey;

// Create a static HashMap indexed by ticker
pub static LISTED_TOKENS_BY_TICKER: Lazy<HashMap<String, Token>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-tokens.yaml");

    // Parse the config
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-tokens.yaml");

    // Convert Vec<Asset> into HashMap<String, Asset>
    config
        .tokens
        .into_iter()
        .map(|asset| (asset.ticker.clone(), asset))
        .collect()
});

// Create a static HashMap indexed by ticker
pub static LISTED_TOKENS: Lazy<HashMap<Pubkey, Token>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-tokens.yaml");

    // Parse the config
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-tokens.yaml");

    // Convert Vec<Asset> into HashMap<Pubkey, Asset>
    config
        .tokens
        .into_iter()
        .map(|asset| (asset.mint, asset))
        .collect()
});

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Token {
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
    tokens: Vec<Token>,
}

impl Token {
    pub fn ore() -> Self {
        LISTED_TOKENS_BY_TICKER.get("ORE").cloned().unwrap()
    }

    pub fn sol() -> Self {
        LISTED_TOKENS_BY_TICKER.get("SOL").cloned().unwrap()
    }

    pub fn usdc() -> Self {
        LISTED_TOKENS_BY_TICKER.get("USDC").cloned().unwrap()
    }

    pub fn _is_ore(&self) -> bool {
        self.ticker == "ORE"
    }

    pub fn is_sol(&self) -> bool {
        self.ticker == "SOL"
    }

    pub fn _is_usdc(&self) -> bool {
        self.ticker == "USDC"
    }
}
