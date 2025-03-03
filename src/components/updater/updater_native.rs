use std::sync::Arc;

use anyhow::Result;
use cargo_packager_updater::Update;
use dioxus::prelude::*;

use crate::{
    components::{Col, Row},
    hooks::{use_updater, UpdateState},
};

pub fn Updater() -> Element {
    let mut updater = use_updater();
    rsx! {
        match &*updater.read() {
            Some(Ok(state)) => {
                match state {
                    UpdateState::AlreadyHaveLatest => {
                        rsx! {}
                    }
                    UpdateState::UpdateAvailable(update, binary) => {
                        let update = update.clone();
                        let version = &update.version;
                        let version = format!("");
                        let binary = Arc::clone(&binary);
                        rsx! {
                            Col {
                                class: "fixed bottom-4 left-4 ml-auto z-100 bg-surface-floating p-4 rounded border-l-4 border-elements-lowEmphasis",
                                gap: 2,
                                span {
                                    class: "text-elements-highEmphasis font-medium",
                                    "A new version of ORE is available!"
                                }
                                Row {

                                    // Install button
                                    button {
                                        class: "text-elements-lowEmphasis hover:cursor-pointer",
                                        onclick: move |_| {
                                            let update = update.clone();
                                            let binary = Arc::clone(&binary);
                                            spawn(async move {
                                                log::info!("installing update");
                                                if let Err(err) = update.install(binary.as_ref()) {
                                                    log::error!("error installing update");
                                                    log::error!("{:?}", err);
                                                } else {
                                                    log::info!("update install complete");
                                                    match std::process::Command::new("open")
                                                        .arg("-n")
                                                        .arg(update.extract_path)
                                                        .spawn()
                                                    {
                                                        Ok(child) => {
                                                            log::info!("process for new app version started: {:?}", child);
                                                            std::process::exit(0);
                                                        }
                                                        Err(err) => {
                                                            log::error!("{:?}", err);
                                                        }
                                                    }
                                                }
                                            });
                                        },
                                        "Install"
                                    }

                                    // Ignore button
                                    button {
                                        class: "text-elements-lowEmphasis hover:cursor-pointer",
                                        onclick: move |_| {
                                            updater.clear();
                                            updater.cancel();
                                        },
                                        "Ignore"
                                    }
                                }
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
