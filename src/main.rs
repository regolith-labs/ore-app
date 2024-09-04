#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod components;
mod gateway;
mod hooks;
mod metrics;
mod miner;
mod route;
mod utils;

use crate::{
    hooks::{
        use_appearance_provider, use_escrow_provider, use_explorer_provider,
        use_power_level_provider, use_wallet_adapter::use_wallet_adapter_provider,
    },
    route::Route,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

// TODO Migration plan
//      Rewrite all components to ignore escrow account.
//      In balance section, check if user has an escrow account.
//      If so, display a "Migrate" button

// TODO Migration plan
//      We have two apps...
//      Refactor entire app as it currently exists into an App component lib
//      If escrow account exists, then display that App
//      If escrow account does not exist, then display this App
//      Add migration flow to the escrow app

pub fn App() -> Element {
    // Global variables
    use_appearance_provider();
    use_explorer_provider();
    use_escrow_provider();
    use_power_level_provider();
    use_wallet_adapter_provider();

    // Render
    rsx! {
        Router::<Route> {}
    }
}
