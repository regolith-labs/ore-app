#![allow(non_snake_case)]
mod components;
mod gateway;
mod hooks;
mod route;
mod steel_app;
mod utils;

use dioxus::prelude::*;
use tracing::Level;

use crate::{
    hooks::{use_explorer_provider, use_wallet_adapter::use_wallet_adapter_provider},
    route::Route,
};

#[cfg(feature = "web")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[cfg(feature = "desktop")]
fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    // Global variables
    use_explorer_provider();
    use_wallet_adapter_provider();

    // Render
    rsx! {
        Router::<Route> {}
    }
}
