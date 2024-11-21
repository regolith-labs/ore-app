use std::str::FromStr;

use dioxus::prelude::*;
use serde::{Deserialize, Deserializer};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;

use crate::steel_app::solana::sdk::pubkey::Pubkey;

pub fn use_assets() -> Resource<Vec<Asset>> {
    use_resource(move || async move {
        let window = web_sys::window().expect("no window exists");
        let resp = JsFuture::from(window.fetch_with_str("/listed-tokens.yaml"))
            .await
            .expect("Failed to fetch config");
        
        let resp: web_sys::Response = resp.dyn_into().unwrap();
        let text = JsFuture::from(resp.text().unwrap())
            .await
            .expect("Failed to get response text")
            .as_string()
            .unwrap();
        log::info!("Got text: {}", text);

        let config: Config = serde_yaml::from_str(&text)
            .expect("Failed to parse config");
        config.assets
    })
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Asset {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub mint: Pubkey,
    pub name: String,
    pub ticker: String,
    pub description: String,
    pub image: String,
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