use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use solana_client_wasm::solana_sdk::signer::Signer;

use crate::{
    components::{
        ActivityIndicator, IsToolbarOpen, MinerDisplayHash, Spinner, StopButton, WarningIcon,
    },
    gateway::signer,
    hooks::{use_gateway, use_power_level, use_priority_fee, PowerLevel, PriorityFee},
    metrics::{track, AppEvent},
    miner::Miner,
    route::Route,
};

use super::MinerStatusMessage;

pub fn MinerToolbarActive() -> Element {
    let gateway = use_gateway(cx);
    let time_remaining = use_state(cx, || 0);
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let miner_status_message = *use_shared_state::<MinerStatusMessage>(cx).unwrap().read();
    let miner_display_hash = use_shared_state::<MinerDisplayHash>(cx)
        .unwrap()
        .read()
        .0
        .to_string();

    use_future(|_| {
        let signer = signer();
        let gateway = gateway.clone();
        let time_remaining = time_remaining.clone();
        async move {
            let proof = gateway.get_proof(signer.pubkey()).await.unwrap();
            let clock = gateway.get_clock().await.unwrap();
            let cutoff_time = proof
                .last_hash_at
                .saturating_add(60)
                .saturating_sub(clock.unix_timestamp)
                .max(0) as u64;
            time_remaining.set(cutoff_time);
        }
    });

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col grow w-full gap-4 px-4 py-6 sm:px-8",
                div {
                    class: "flex flex-col w-full gap-2",
                    div {
                        class: "flex flex-row w-full justify-between",
                        h2 {
                            class: "text-3xl md:text-4xl lg:text-5xl text-white font-bold",
                            "Mining"
                        }
                        div {
                            class: "my-auto",
                            StopButton {
                                miner: cx.props.miner.clone()
                            }
                        }
                    }
                    match miner_status_message {
                        MinerStatusMessage::Searching => {
                            render! {
                                p {
                                    class: "text-lg text-white",
                                    "Searching for a valid hash... ({time_remaining} sec)"
                                }
                            }
                        }
                        MinerStatusMessage::Submitting => {
                            render! {
                                div {
                                    class: "flex flex-row gap-2",
                                    p {
                                        class: "text-lg text-white",
                                        "Submitting hash for validation..."
                                    }
                                    Spinner {
                                        class: "my-auto"
                                    }
                                }
                            }
                        }
                        MinerStatusMessage::Error => {
                            render! {
                                p {
                                    class: "text-lg text-white",
                                    "Error submitting transaction"
                                }
                            }
                        }
                        _ => None
                    }
                    match miner_status_message {
                        MinerStatusMessage::Searching | MinerStatusMessage::Submitting => {
                            render! {
                                p {
                                    class: "font-mono text-sm truncate shrink opacity-80",
                                    "{miner_display_hash}"
                                }
                            }
                        }
                        _ => None
                    }
                }
                PriorityFeeConfig {}
                PowerLevelConfig {}
                DownloadLink {}
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row gap-2 max-w-screen w-screen justify-start my-auto px-4 sm:px-8 object-contain",
                div {
                    class: "flex-shrink-0 flex-none my-auto",
                    ActivityIndicator {}
                }
                p {
                    class: "font-semibold text-white flex-shrink-0 flex-none my-auto",
                    "Mining"
                }
                div {
                    class: "flex-shrink flex-auto truncate my-auto",
                    match miner_status_message {
                        MinerStatusMessage::Searching => {
                            render! {
                                p {
                                    class: "font-mono text-sm truncate flex-shrink flex-auto opacity-80 my-auto ml-2",
                                    "{miner_display_hash}"
                                }
                            }
                        }
                        MinerStatusMessage::Submitting => {
                            render! {
                                p {
                                    class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                    "Submitting hash for validation..."
                                }
                            }
                        }
                        MinerStatusMessage::Error => {
                            render! {
                                p {
                                    class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                    "Error submitting transaction"
                                }
                            }
                        }
                        _ => None
                    }
                }
                div {
                    class: "flex-shrink-0 flex-none ml-auto my-auto",
                    StopButton {
                        miner: cx.props.miner.clone()
                    }
                }
            }
        }
    }
}

#[component]
pub fn PriorityFeeConfig(cx: Scope) -> Element {
    let priority_fee = use_priority_fee(cx);
    render! {
        div {
            class: "flex flex-row gap-8 justify-between mt-8",
            div {
                class: "flex flex-col gap-1",
                p {
                    class: "text-white font-semibold",
                    "Priority fee"
                }
                p {
                    class: "text-white text-xs opacity-80 max-w-96",
                    "When Solana is busy, priority fees can increase the chances of your transactions being accepted."
                }

            }
            div {
                class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                input {
                    class: "bg-transparent text-white text-right px-1 mb-auto",
                    step: 100_000,
                    min: 0,
                    max: 10_000_000,
                    r#type: "number",
                    value: "{priority_fee.read().0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value.parse::<u64>() {
                            track(AppEvent::SetPriorityFee, None);
                            *priority_fee.write() = PriorityFee(v);
                        }
                    }
                }
                p {
                    class: "my-auto",
                    "microlamports"
                }
            }
        }
    }
}

#[component]
pub fn PowerLevelConfig(cx: Scope) -> Element {
    let power_level = use_power_level(cx);
    if cfg!(feature = "web") {
        return None;
    }
    render! {
        div {
            class: "flex flex-row gap-8 justify-between mt-8",
            div {
                class: "flex flex-col gap-1",
                p {
                    class: "text-white font-semibold",
                    "Power level"
                }
                p {
                    class: "text-white text-xs opacity-80 max-w-96",
                    "Configure how much of your device's computing capacity to utilize while mining."
                }

            }
            div {
                class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                input {
                    class: "bg-transparent text-white text-right px-1 mb-auto",
                    disabled: cfg!(feature = "web"),
                    step: 10,
                    min: 10,
                    max: 100,
                    r#type: "number",
                    value: "{power_level.read().0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value.parse::<u64>() {
                            *power_level.write() = PowerLevel(v as u8);
                        }
                    }
                }
                p {
                    class: "my-auto",
                    "%"
                }
            }
        }
    }
}

#[component]
fn DownloadLink(cx: Scope) -> Element {
    if cfg!(feature = "web") {
        render! {
            div {
                class: "flex flex-row gap-2 mt-8 p-2.5 rounded bg-green-600",
                WarningIcon {
                    class: "w-4 h-4 mt-0.5 shrink-0"
                }
                p {
                    class: "text-sm my-auto",
                    "You are mining from a web browser. For better performance, "
                    Link {
                        to: Route::Download {},
                        class: "font-medium underline",
                        "download the app."
                    }
                }
            }
        }
    } else {
        None
    }
}
