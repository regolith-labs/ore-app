use std::sync::Arc;

use anyhow::Result;
use cargo_packager_updater::{semver::Version, url::Url, Config, Update};
use dioxus::prelude::*;

// releases endpoint
const ENDPOINT: &str = "https://api.ore.supply/app/update/{{target}}/{{arch}}/{{current_version}}";
// releases signer pubkey
const PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEE1RkFDQUFCQ0M0NDhBRTQKUldUa2lrVE1xOHI2cGJSaXdCS0NVWGdBQTYzSGFNTXlBRlc5NThYVFhwUEVab29UaGpiSk1WWloK";

#[derive(Clone, Debug)]
pub enum UpdateState {
    AlreadyHaveLatest,
    UpdateAvailable(Update, Arc<NewBinaryToInstall>),
}
pub type NewBinaryToInstall = Vec<u8>;

pub fn use_updater() -> Resource<Result<UpdateState>> {
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
        let (sender, receiver) = tokio::sync::oneshot::channel::<UpdateState>();
        // check for update
        let handle = std::thread::spawn(move || {
            let update = cargo_packager_updater::check_update(current_version, config);
            let state = if let Ok(Some(update)) = update {
                log::info!("update found: {:?}", update);
                // download
                if let Ok(bytes) = update.download() {
                    log::info!("update downloaded");
                    UpdateState::UpdateAvailable(update, Arc::new(bytes))
                } else {
                    log::error!("failed to download latest update");
                    UpdateState::AlreadyHaveLatest
                }
            } else {
                log::info!("no update available");
                UpdateState::AlreadyHaveLatest
            };
            if let Err(_err) = sender.send(state) {
                log::error!("error sending download to signal");
            }
        });
        // spawn dioxus thread to listen for update
        receiver.await.map_err(|err| anyhow::anyhow!(err))
    })
}
