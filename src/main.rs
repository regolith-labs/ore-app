#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use gateway::Gateway;
#[cfg(feature = "web")]
use web_sys::window;

mod components;
#[cfg(feature = "desktop")]
mod file;
mod gateway;
mod hooks;
mod metrics;
mod miner;
mod route;
#[cfg(feature = "web")]
mod worker;

#[cfg(feature = "web")]
use crate::{components::Appearance, hooks::use_appearance};
use crate::{
    hooks::{
        use_appearance_provider, use_explorer_provider, use_is_onboarded_provider,
        use_ore_balance_provider, use_power_level_provider, use_priority_fee_provider,
        use_proof_provider, use_sol_balance_provider, ProofHandle,
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
    env_logger::init();
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()),
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

    // Gateway
    use_context_provider(cx, || Rc::new(Gateway::new()));

    // Network variables
    use_proof_provider(cx);
    use_ore_balance_provider(cx);
    use_sol_balance_provider(cx);

    // Dark mode appearance
    #[cfg(feature = "web")]
    {
        let appearance = use_appearance(cx);
        use_effect(cx, appearance, |_| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let classname = match *appearance.read() {
                            Appearance::Dark => "dark",
                            Appearance::Light => "",
                        };
                        body.set_class_name(classname);
                    }
                }
            }
            async move {}
        });
    }

    // Render
    render! {
        Router::<Route> {}
    }
}
