#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use gateway::Gateway;

mod components;
mod gateway;
mod hooks;
mod route;

#[cfg(feature = "web")]
use crate::gateway::find_next_hash;
use crate::{
    hooks::{use_appearance_persistant, use_explorer_persistant},
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
    // Appearance
    let appearance = use_appearance_persistant(cx).get();
    use_shared_state_provider(cx, || appearance);

    // Explorer
    let explorer = use_explorer_persistant(cx).get();
    use_shared_state_provider(cx, || explorer);

    // Gateway
    use_context_provider(cx, || Rc::new(Gateway::new()));

    // Render
    render! {
        Router::<Route> {}
    }
    // render! {
    //     p {
    //         "hello world"
    //     }
    // }
}
