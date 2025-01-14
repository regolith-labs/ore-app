#![allow(non_snake_case)]
mod components;
mod gateway;
mod hooks;
mod pages;
mod route;
mod steel_app;
mod utils;

use dioxus::prelude::*;
use tracing::Level;

use crate::{
    components::MinerController,
    hooks::{use_miner_is_active_provider, use_miner_provider, use_wallet_provider},
    route::Route,
};

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    #[cfg(feature = "web")]
    launch(App);
}

pub fn App() -> Element {
    use_miner_provider();
    use_miner_is_active_provider();
    use_wallet_provider();
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Link { rel: "icon", href: asset!("/public/icon.png") }
        document::Script { src: asset!("/public/wallet.js") }
        MinerController {}
        Router::<Route> {}
    }
}
