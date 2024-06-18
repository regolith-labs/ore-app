use std::{fmt, io, str::FromStr};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

const KEY: &str = "explorer";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum Explorer {
    #[default]
    Solana,
    SolanaFm,
    Solscan,
    Xray,
}

impl fmt::Display for Explorer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Explorer::Solana => write!(f, "Solana Explorer"),
            Explorer::SolanaFm => write!(f, "SolanaFM"),
            Explorer::Solscan => write!(f, "Solscan"),
            Explorer::Xray => write!(f, "Xray"),
        }
    }
}

impl FromStr for Explorer {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Solana Explorer" => Ok(Explorer::Solana),
            "SolanaFM" => Ok(Explorer::SolanaFm),
            "Solscan" => Ok(Explorer::Solscan),
            "Xray" => Ok(Explorer::Xray),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown explorer",
            )),
        }
    }
}

pub fn use_explorer() -> Signal<Explorer> {
    let explorer = use_context::<Signal<Explorer>>();
    let mut explorer_persistent = use_persistent(KEY, || Explorer::Solana);
    use_effect(move || explorer_persistent.set(*explorer.read()));
    explorer
}

pub fn use_explorer_provider() {
    let explorer = use_persistent(KEY, || Explorer::Solana).get();
    use_context_provider(|| Signal::new(explorer));
}

pub fn use_explorer_account_url(address: String) -> String {
    let explorer = use_explorer();
    let e = *explorer.read();
    match e {
        Explorer::Solana => format!("https://explorer.solana.com/address/{}", address),
        Explorer::SolanaFm => format!("https://solana.fm/address/{}", address),
        Explorer::Solscan => format!("https://solscan.io/account/{}", address),
        Explorer::Xray => format!("https://xray.helius.xyz/account/{}", address),
    }
}

pub fn use_explorer_transaction_url(signature: String) -> String {
    let explorer = use_explorer();
    let e = *explorer.read();
    match e {
        Explorer::Solana => format!("https://explorer.solana.com/tx/{}", signature),
        Explorer::SolanaFm => format!("https://solana.fm/tx/{}", signature),
        Explorer::Solscan => format!("https://solscan.io/tx/{}", signature),
        Explorer::Xray => format!("https://xray.helius.xyz/tx/{}", signature),
    }
}
