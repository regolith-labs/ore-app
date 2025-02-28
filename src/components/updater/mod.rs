use std::sync::Arc;

use dioxus::prelude::*;

use anyhow::Result;
use cargo_packager_updater::{semver::Version, url::Url, Config, Update};

// releases endpoint
const ENDPOINT: &str = "https://api.ore.supply/app/update/{{target}}/{{arch}}/{{current_version}}";
// releases signer pubkey
const PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEE1RkFDQUFCQ0M0NDhBRTQKUldUa2lrVE1xOHI2cGJSaXdCS0NVWGdBQTYzSGFNTXlBRlc5NThYVFhwUEVab29UaGpiSk1WWloK";

pub fn Updater() -> Element {
    let signal = use_signal(|| State::AlreadyHaveLatest);
    updater(signal);
    rsx! {
        match signal.cloned() {
            State::AlreadyHaveLatest => {
                rsx! {}
            }
            State::UpdateAvailable(update, binary) => {
                let update = update.clone();
                let binary = Arc::clone(&binary);
                rsx! {
                    button {
                        class: "text-xl",
                        onclick: move |_| {
                            let update = update.clone();
                            let binary = Arc::clone(&binary);
                            spawn(async move {
                                if let Err(err) = update.install(binary) {
                                    log::error!("{:?}", err);
                                }
                            });
                        },
                        "update?"
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
enum State {
    AlreadyHaveLatest,
    UpdateAvailable(Update, Arc<NewBinaryToInstall>),
}
type NewBinaryToInstall = Vec<u8>;

fn updater(mut signal: Signal<State>) -> Result<()> {
    let endpoint = Url::parse(ENDPOINT)?;
    // signer pubkey
    let pubkey = String::from(PUBKEY);
    // config
    let config = Config {
        endpoints: vec![endpoint],
        pubkey,
        ..Default::default()
    };
    // current version for reference
    let current_version = env!("CARGO_PKG_VERSION");
    let current_version = Version::parse(current_version)?;
    // channel
    let (sender, receiver) = tokio::sync::oneshot::channel::<State>();
    // check for update
    let handle = std::thread::spawn(move || {
        let update = cargo_packager_updater::check_update(current_version, config);
        let state = if let Ok(Some(update)) = update {
            log::info!("update: {:?}", update);
            // download
            if let Ok(bytes) = update.download() {
                State::UpdateAvailable(update, Arc::new(bytes))
            } else {
                log::error!("failed to download latest update");
                State::AlreadyHaveLatest
            }
        } else {
            log::info!("no update available");
            State::AlreadyHaveLatest
        };
        if let Err(err) = sender.send(state) {
            log::error!("{:?}", err);
        }
    });
    // spawn dioxus thread to listen for update
    spawn(async move {
        match receiver.await {
            Ok(state) => {
                signal.set(state);
            }
            Err(err) => {
                log::error!("{:?}", err);
            }
        }
    });
    Ok(())
}
