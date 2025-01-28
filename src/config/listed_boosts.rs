use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;
use steel::Pubkey;

use crate::utils::deserialize_pubkey;

// Create a static HashMap indexed by ticker
pub static LISTED_BOOSTS: Lazy<Vec<BoostMeta>> = Lazy::new(|| {
    let yaml_str = include_str!("../../public/config/listed-boosts.yaml");
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-boosts.yaml");
    config.boosts
});

pub static LISTED_BOOSTS_BY_MINT: Lazy<HashMap<Pubkey, BoostMeta>> = Lazy::new(|| {
    let yaml_str = include_str!("../../public/config/listed-boosts.yaml");
    let config: Config =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-boosts.yaml");
    config.boosts
        .into_iter()
        .map(|boost| (boost.lp_mint, boost))
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
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub lp_id: Pubkey,
    pub lp_type: LpType,
    pub name: String,
    pub ticker: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum LpType {
    #[serde(rename = "kamino")]
    Kamino,
    #[serde(rename = "meteora")]
    Meteora,
}

impl std::fmt::Display for LpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LpType::Kamino => write!(f, "Kamino"),
            LpType::Meteora => write!(f, "Meteora"),
        }
    }
}

#[derive(Deserialize)]
struct Config {
    boosts: Vec<BoostMeta>,
}
