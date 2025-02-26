use dioxus::prelude::*;

use anyhow::Result;
use cargo_packager_updater::{semver::Version, url::Url, Config, Update};

// releases endpoint
const ENDPOINT: &str = "http://localhost:3000/app/update/{{target}}/{{arch}}/{{current_version}}";

pub fn Updater() -> Element {
    let state = use_resource(move || async move { updater() });
    rsx! {
        match &*state.read() {
            Some(Ok(state)) => {
                match state {
                    State::AlreadyHaveLatest => {
                        rsx! {}
                    }
                    State::UpdateAvailable(update, binary) => {
                        let update = update.clone();
                        let binary = binary.clone();
                        rsx! {
                            button {
                                onclick: move |_| {
                                    let update = update.clone();
                                    let binary = binary.clone();
                                    spawn(async move {
                                        if let Err(err) = update.install(binary) {
                                            println!("{:?}", err);
                                        }
                                    });
                                },
                                "update?"
                            }
                        }
                    }
                }
            }
            Some(Err(err)) => {
                rsx! {}
            }
            None => {
                rsx! {}
            }
        }
    }
}

#[derive(Clone)]
enum State {
    AlreadyHaveLatest,
    UpdateAvailable(Update, NewBinaryToInstall),
}
type NewBinaryToInstall = Vec<u8>;

fn updater() -> Result<State> {
    let endpoint = Url::parse(ENDPOINT)?;
    println!("{:?}", endpoint);
    // signer pubkey
    let pubkey = String::from("dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEE1RkFDQUFCQ0M0NDhBRTQKUldUa2lrVE1xOHI2cGJSaXdCS0NVWGdBQTYzSGFNTXlBRlc5NThYVFhwUEVab29UaGpiSk1WWloK");
    // config
    let config = Config {
        endpoints: vec![endpoint],
        pubkey,
        ..Default::default()
    };
    // current version for reference
    let current_version = env!("CARGO_PKG_VERSION");
    let current_version = Version::parse(current_version)?;
    // check for update
    let handle = std::thread::spawn(move || {
        let update = cargo_packager_updater::check_update(current_version, config)?;
        if let Some(update) = update {
            println!("update: {:?}", update);
            // download
            let bytes = update.download()?;
            Ok(State::UpdateAvailable(update, bytes))
        } else {
            println!("no update available");
            Ok(State::AlreadyHaveLatest)
        }
    });
    match handle.join() {
        Ok(res) => res,
        Err(err) => {
            log::error!("{:?}", err);
            Err(anyhow::anyhow!("failed"))
        }
    }
}
