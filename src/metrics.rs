use std::fmt::Display;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// TODO StopMiner
// TODO SetPowerLevel
// TODO SetAppearance
// TODO SetExplorer
// TODO ExportKey
// TODO DownloadApp
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AppEvent {
    Mine,
}

impl Display for AppEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AppEvent::Mine => "Mine",
        };
        write!(f, "{:}", str)
    }
}

// Track an app event.
pub fn track(event: AppEvent) {
    trackEvent(event.to_string().as_str(), None).ok();
}

// Define a function that calls `fathom.trackEvent`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = fathom, catch)]
    fn trackEvent(name: &str, value: Option<u32>) -> Result<(), JsValue>;
}
