#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod components;
mod gateway;
mod hooks;
mod miner;
mod route;
mod utils;
mod wallet_adapter;

use crate::{
    hooks::{
        use_appearance_provider, use_explorer_provider, use_is_onboarded_provider,
        use_power_level_provider, use_priority_fee_provider, use_rpc_url_provider,
        use_show_backup_warning_provider, use_wallet_adapter_provider,
    },
    route::Route,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    // Global variables
    use_appearance_provider();
    use_explorer_provider();
    use_power_level_provider();
    use_is_onboarded_provider();
    use_priority_fee_provider();
    use_show_backup_warning_provider();
    use_rpc_url_provider();
    use_wallet_adapter_provider();

    // Render
    rsx! {
        Router::<Route> {}
    }
}
