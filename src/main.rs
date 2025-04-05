// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
mod components;
mod config;
mod cores;
mod gateway;
mod hooks;
mod logger;
mod pages;
mod route;
mod solana;
mod time;
mod utils;

use dioxus::prelude::*;
#[cfg(feature = "web")]
use hooks::use_download_url_provider;
use tracing::Level;

#[cfg(all(feature = "desktop", target_os = "macos"))]
use crate::utils::AppNapDisabler;
use crate::{
    hooks::{
        use_cache_provider, use_docs_drawer_state_provider, use_miner_cores_provider,
        use_miner_events_provider, use_miner_provider, use_miner_status_provider, use_mining_loop,
        use_transaction_status_provider, use_wallet_drawer_state_provider, use_wallet_provider,
        use_wss_provider,
    },
    route::Route,
};

const CSS: &str = include_str!("../public/tailwind.css");

fn main() {
    logger::init();
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    #[cfg(all(feature = "desktop", target_os = "macos"))]
    let _nap_blocker = AppNapDisabler::new();
    dioxus::launch(App)
}

pub fn App() -> Element {
    #[cfg(feature = "web")]
    use_download_url_provider();
    use_miner_provider();
    use_miner_status_provider();
    use_miner_events_provider();
    use_miner_cores_provider();
    use_transaction_status_provider();
    use_wallet_provider();
    use_wss_provider();
    use_cache_provider();
    use_mining_loop();
    use_wallet_drawer_state_provider();
    use_docs_drawer_state_provider();

    rsx! {
        style { "{CSS}" }
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Link { rel: "icon", href: asset!("/public/icon.png") }
        document::Script { src: asset!("/public/wallet.js") }
        if cfg!(feature = "web") {
            document::Script { src: "https://unpkg.com/@splinetool/viewer/build/spline-viewer.js", r#type: "module" }
            document::Script {
                src: "https://cdn.usefathom.com/script.js",
                r#type: "text/javascript",
                defer: true,
                "data-spa": "auto",
                "data-site": "FCVSKOIN",
            }
        }
        Router::<Route> {}
    }
}
