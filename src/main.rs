#![allow(non_snake_case)]
mod components;
mod gateway;
mod hooks;
mod pages;
mod route;
mod steel_app;
mod utils;

use dioxus::prelude::*;
use tracing::Level;

use crate::{hooks::use_wallet_status_provider, route::Route};

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        check_version();
    }
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    use_wallet_status_provider();
    rsx! {
        Router::<Route> {}
    }
}

#[cfg(feature = "web")]
fn check_version() {
    let git_hash = env!("GIT_HASH");
    
    if let Ok(stored_hash) = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("app_git_hash")
    {
        if stored_hash.as_deref() != Some(&git_hash) {
            web_sys::window()
                .unwrap()
                .location()
                .reload()
                .unwrap();
        }
    }
    
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item("app_git_hash", &git_hash)
        .unwrap();
}
