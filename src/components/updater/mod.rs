use std::sync::Arc;

use dioxus::prelude::*;

use anyhow::Result;
use cargo_packager_updater::{semver::Version, url::Url, Config, Update};

// releases endpoint
const ENDPOINT: &str = "https://api.ore.supply/app/update/{{target}}/{{arch}}/{{current_version}}";
// releases signer pubkey
const PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEE1RkFDQUFCQ0M0NDhBRTQKUldUa2lrVE1xOHI2cGJSaXdCS0NVWGdBQTYzSGFNTXlBRlc5NThYVFhwUEVab29UaGpiSk1WWloK";

pub fn Updater() -> Element {
    let update = updater();
    rsx! {
        match &*update.read() {
            Some(Ok(state)) => {
                match state {
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
                                    log::info!("button clicked");
                                    let update = update.clone();
                                    let binary = Arc::clone(&binary);
                                    spawn(async move {
                                        log::info!("installing");
                                        if let Err(err) = update.install(binary.as_ref()) {
                                            log::error!("error installing binary");
                                            log::error!("{:?}", err);
                                        } else {
                                            log::info!("install complete");
                                            match std::process::Command::new("open")
                                                .arg("-n")
                                                .arg(update.extract_path)
                                                .spawn()
                                            {
                                                Ok(child) => {
                                                    log::info!("new process started: {:?}", child);
                                                    std::process::exit(0);
                                                }
                                                Err(err) => {
                                                    log::error!("{:?}", err);
                                                }
                                            }
                                        }
                                    });
                                },
                                "update?"
                            }
                        }
                    }
                }
            }
            _ => {
                rsx! {}
            }
        }
    }
}

#[derive(Clone, Debug)]
enum State {
    AlreadyHaveLatest,
    UpdateAvailable(Update, Arc<NewBinaryToInstall>),
}
type NewBinaryToInstall = Vec<u8>;

fn updater() -> Resource<Result<State>> {
    use_resource(move || async move {
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
        // update channel --
        // the updater is implemented with a blocking client
        // so to fetch the binary to install without blocking the UI
        // we need to spawn a system thread
        // and then send the bytes thru a oneshot channel
        // which supports a sync sender *and* async reader
        let (sender, receiver) = tokio::sync::oneshot::channel::<State>();
        // check for update
        let handle = std::thread::spawn(move || {
            let update = cargo_packager_updater::check_update(current_version, config);
            let state = if let Ok(Some(update)) = update {
                log::info!("update found: {:?}", update);
                // download
                if let Ok(bytes) = update.download() {
                    log::info!("update downloaded");
                    State::UpdateAvailable(update, Arc::new(bytes))
                } else {
                    log::error!("failed to download latest update");
                    State::AlreadyHaveLatest
                }
            } else {
                log::info!("no update available");
                State::AlreadyHaveLatest
            };
            if let Err(_err) = sender.send(state) {
                log::error!("error sending download to signal");
            }
        });
        // spawn dioxus thread to listen for update
        receiver.await.map_err(|err| anyhow::anyhow!(err))
    })
}
