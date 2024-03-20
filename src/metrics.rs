use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

// TODO StopMiner
// TODO SetPowerLevel
// TODO SetAppearance
// TODO SetExplorer
// TODO ExportKey
// TODO DownloadApp
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AppEvent {
    CreateTokenAccount,
    Register,
    StartMiner,
    StopMiner,
    Claim,
    Transfer,
    SetPriorityFee,
}

impl Display for AppEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AppEvent::CreateTokenAccount => "Create token accont",
            AppEvent::Register => "Register",
            AppEvent::StartMiner => "Start miner",
            AppEvent::StopMiner => "Stop miner",
            AppEvent::Claim => "Claim",
            AppEvent::Transfer => "Transfer",
            AppEvent::SetPriorityFee => "Set priority fee",
        };
        write!(f, "{:}", str)
    }
}

// Define a function that calls `fathom.trackEvent`
#[cfg(feature = "web")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = fathom, catch)]
    fn trackEvent(name: &str, value: Option<u32>) -> Result<(), JsValue>;
}

#[cfg(feature = "web")]
pub fn track(event: AppEvent, value: Option<u32>) {
    trackEvent(event.to_string().as_str(), value).ok();
}

#[cfg(feature = "desktop")]
pub fn track(_event: AppEvent, _value: Option<u32>) {
    // Noop
}
