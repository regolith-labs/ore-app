#![allow(non_snake_case)]
mod components;
mod config;
mod gateway;
mod hooks;
mod pages;
mod route;
mod time;
mod utils;

use dioxus::prelude::*;
use hooks::{use_miner_status_provider, use_mining_loop};
use tracing::Level;

use crate::{
    hooks::{use_cache_provider, use_transaction_status_provider, use_miner_provider, use_wallet_provider},
    route::Route,
};

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
    use_transaction_status_provider();
    use_wallet_provider();
    use_cache_provider();
    use_mining_loop();
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css"), blocking: "render" }
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Link { rel: "icon", href: asset!("/public/icon.png") }
        document::Script { src: asset!("/public/wallet.js") }
        Router::<Route> {}
    }
}
