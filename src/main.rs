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

const RPC_URL: &str = "https://devnet.helius-rpc.com/?api-key=bb9df66a-8cba-404d-b17a-e739fe6a480c";
const API_URL: &str = "https://ore-api-lthm.onrender.com/transfers";
const WSS_URL: &str = "wss://ore-websockets.onrender.com/ws";

#[component]
fn App(cx: Scope) -> Element {
    use_context_provider(cx, || {
        Rc::new(Gateway::new(
            API_URL.to_string(),
            RPC_URL.to_string(),
            WSS_URL.to_string(),
        ))
    });
    render! {
        Router::<Route> {}
    }
}
