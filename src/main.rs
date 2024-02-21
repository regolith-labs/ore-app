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
use crate::route::Route;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    use_context_provider(cx, || Rc::new(Gateway::new()));
    render! {
        Router::<Route> {}
    }
}
