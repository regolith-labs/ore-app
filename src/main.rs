#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
#[cfg(feature = "desktop")]
mod file;
mod gateway;
mod hooks;
mod metrics;
mod miner;
mod route;
mod utils;
#[cfg(feature = "web")]
mod worker;

use crate::{
    hooks::{
        use_appearance_provider, use_explorer_provider, use_is_onboarded_provider,
        use_ore_balance_provider, use_power_level_provider, use_priority_fee_provider,
        use_proof_provider, use_rpc_url_provider, use_show_backup_warning_provider,
        use_sol_balance_provider, ProofHandle,
    },
    route::Route,
};

#[cfg(feature = "web")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

#[cfg(feature = "desktop")]
fn main() {
    use utils::asset_path;
    env_logger::init();
    let tailwind_path = asset_path("tailwind.css");
    let custom_head = format!(r#"<link rel="stylesheet" href="{}">"#, tailwind_path);
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_custom_head(custom_head),
    );
}

#[component]
fn App(cx: Scope) -> Element {
    // Global variables
    use_appearance_provider(cx);
    use_explorer_provider(cx);
    use_power_level_provider(cx);
    use_is_onboarded_provider(cx);
    use_priority_fee_provider(cx);
    use_show_backup_warning_provider(cx);
    use_rpc_url_provider(cx);

    // Network variables
    use_proof_provider(cx);
    use_ore_balance_provider(cx);
    use_sol_balance_provider(cx);

    // Render
    render! {
        Router::<Route> {}
    }
}
