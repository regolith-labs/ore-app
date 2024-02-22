#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_router::components::Router;
use gateway::Gateway;

mod components;
mod gateway;
mod hooks;
mod route;

pub use crate::gateway::find_next_hash;
use crate::{
    hooks::{use_appearance_persistant, use_explorer_persistant},
    route::Route,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
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
}
