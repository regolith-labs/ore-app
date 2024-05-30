#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod gateway;
mod hooks;
// mod metrics;
mod miner;
mod route;
mod utils;

use crate::{
    hooks::{
        use_appearance_provider,
        // use_explorer_provider,
        use_is_onboarded_provider,
        //         use_ore_balance_provider,
        use_power_level_provider,
        use_priority_fee_provider,
        //         use_proof_provider, use_show_backup_warning_provider,
        //         use_sol_balance_provider, ProofHandle,
        use_rpc_url_provider,
    },
    route::Route,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}

pub fn App() -> Element {
    // Global variables
    use_appearance_provider();
    // use_explorer_provider(cx);
    use_power_level_provider();
    use_is_onboarded_provider();
    use_priority_fee_provider();
    // use_show_backup_warning_provider(cx);
    use_rpc_url_provider();

    // Network variables
    // use_proof_provider(cx);
    // use_ore_balance_provider(cx);
    // use_sol_balance_provider(cx);

    // Render
    rsx! {
        Router::<Route> {}
    }
}
