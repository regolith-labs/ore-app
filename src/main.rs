#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Router;

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
    render! {
        Router::<Route> {}
    }
}
