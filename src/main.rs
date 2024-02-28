#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use gateway::Gateway;

mod components;
#[cfg(feature = "desktop")]
mod file;
mod gateway;
mod hooks;
mod miner;
mod route;
#[cfg(feature = "web")]
mod worker;

use crate::{
    hooks::{use_appearance_provider, use_explorer_provider, use_power_level_provider},
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
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()),
    );
}

#[component]
fn App(cx: Scope) -> Element {
    // Global variables
    use_appearance_provider(cx);
    use_explorer_provider(cx);
    use_power_level_provider(cx);

    // Gateway
    use_context_provider(cx, || Rc::new(Gateway::new()));

    // Render
    render! {
        Router::<Route> {}
    }
}
