#![allow(non_snake_case)]
mod components;
mod config;
mod cores;
mod gateway;
mod hooks;
mod pages;
mod route;
mod solana;
mod time;
mod utils;

use dioxus::prelude::*;
use hooks::{use_miner_cores_provider, use_miner_status_provider, use_mining_loop};
use tracing::Level;

use crate::{
    hooks::{
        use_cache_provider, use_miner_events_provider, use_miner_provider,
        use_transaction_status_provider, use_wallet_provider,
    },
    route::Route,
};

const CSS: &str = include_str!("../public/tailwind.css");

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());

    use directories::ProjectDirs;
    use log::{info, warn};
    use std::fs::{create_dir_all, OpenOptions};

    use fern::Dispatch;
    use log::LevelFilter;
    use std::fs;
    use std::path::PathBuf;

    // Let's say your app's "organization" is "myorg" and app name is "MyApp".
    // The domain qualifier can be "com", or your domain, etc.
    let proj_dirs =
        ProjectDirs::from("ore", "supply", "ore-app").expect("Could not determine home directory");

    // For logs, it's common to use `cache_dir` or `data_local_dir`, but choose whichever fits:
    let log_dir: PathBuf = proj_dirs.cache_dir().join("logs");
    fs::create_dir_all(&log_dir).expect("Could not create log directory");

    let log_file_path = log_dir.join("my_logs.log");

    // Use fern (or another crate) to write logs to file (and optionally to stdout too)
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(&log_file_path).expect("Failed to open log file"))
        .apply()
        .expect("Failed to set up logging");

    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element {
    use_miner_provider();
    use_miner_status_provider();
    use_miner_events_provider();
    use_miner_cores_provider();
    use_transaction_status_provider();
    use_wallet_provider();
    use_cache_provider();
    use_mining_loop();
    rsx! {
        style { "{CSS}" }
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Link { rel: "icon", href: asset!("/public/icon.png") }
        document::Script { src: asset!("/public/wallet.js") }
        Router::<Route> {}
    }
}
