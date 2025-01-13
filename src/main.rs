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

use crate::{hooks::use_wallet_provider, route::Route};

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    #[cfg(feature = "web")]
    launch(App);
}

pub fn App() -> Element {
    use_wallet_provider();
    rsx! {
        // Icons
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Link { rel: "icon", href: asset!("/public/icon.png") }

        // Tailwind
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }

        // Scripts
        document::Script { src: asset!("/public/wallet.js") }

        Router::<Route> {}
    }
}
