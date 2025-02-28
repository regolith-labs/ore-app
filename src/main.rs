#![allow(non_snake_case)]
mod components;
mod config;
mod cores;
mod gateway;
mod hooks;
mod pages;
mod route;
mod solana;
mod time;
mod utils;

use dioxus::prelude::*;
use hooks::{use_miner_cores_provider, use_miner_status_provider, use_mining_loop};
use tracing::Level;

use crate::{
    hooks::{
        use_cache_provider, use_miner_events_provider, use_miner_provider,
        use_transaction_status_provider, use_wallet_provider,
    },
    route::Route,
};

const CSS: &str = include_str!("../public/tailwind.css");

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());
    #[cfg(not(feature = "web"))]
    env_logger::init();
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    use_miner_provider();
    use_miner_status_provider();
    use_miner_events_provider();
    use_miner_cores_provider();
    use_transaction_status_provider();
    use_wallet_provider();
    use_cache_provider();
    use_mining_loop();
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
