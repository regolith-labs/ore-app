#[cfg(feature = "desktop")]
use std::time::Duration;

use chrono::Utc;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use ore::START_AT;
#[cfg(feature = "web")]
use web_time::Duration;

use crate::{
    components::{
        format_duration, ActivityIndicator, IsToolbarOpen, MinerDisplayHash, Spinner, StopButton,
        WarningIcon,
    },
    hooks::{use_power_level, use_priority_fee, PowerLevel, PriorityFee},
    metrics::{track, AppEvent},
    miner::Miner,
    route::Route,
};

use super::MinerStatusMessage;

#[derive(Props, PartialEq)]
pub struct MinerToolbarActiveProps {
    pub miner: UseState<Miner>,
}

#[component]
pub fn MinerToolbarActive(cx: Scope<MinerToolbarActiveProps>) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let miner_status_message = *use_shared_state::<MinerStatusMessage>(cx).unwrap().read();
    let miner_display_hash = use_shared_state::<MinerDisplayHash>(cx)
        .unwrap()
        .read()
        .0
        .to_string();

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
                                    "Searching for a valid hash..."
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
                        MinerStatusMessage::Waiting => {
                            render! {
                                div {
                                    class: "flex flex-row gap-1",
                                    p {
                                        class: "text-lg text-white my-auto",
                                        "Mining will start in"
                                    }
                                    CountdownTimer {
                                        class: "text-lg text-white my-auto"
                                    }
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
                        MinerStatusMessage::Waiting => {
                            render! {
                                div {
                                    class: "flex flex-row gap-1 shrink w-min justify-start",
                                    p {
                                        class: "truncate w-min shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                        "Mining will start in "
                                    }
                                    CountdownTimer {
                                        class: "text-sm text-white opacity-80 my-auto"
                                    }
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
pub fn CountdownTimer<'a>(cx: Scope, class: Option<&'a str>) -> Element {
    let class = class.unwrap_or("");

    let miner_status_message = use_shared_state::<MinerStatusMessage>(cx).unwrap();
    let now_unix_timestamp = Utc::now().timestamp();
    let seconds_until_launch = START_AT - now_unix_timestamp;
    let time = use_state(cx, || seconds_until_launch);
    let t = format_duration(*time.get());

    // Countdown until launch
    use_future(cx, (), |_| {
        let miner_status_message = miner_status_message.clone();
        let time = time.clone();
        async move {
            loop {
                async_std::task::sleep(Duration::from_secs(1)).await;
                let now_unix_timestamp = Utc::now().timestamp();
                let seconds_until_launch = START_AT - now_unix_timestamp;
                time.set(seconds_until_launch);
                if seconds_until_launch < 0 {
                    *miner_status_message.write() = MinerStatusMessage::Searching;
                    break;
                }
            }
        }
    });

    render! {
        p {
            class: "{class}",
            "{t}"
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
                    step: 1,
                    min: 0,
                    max: 1000,
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
                    "lamports"
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
                    "You are mining from a web browser which can lead to inconsistent results. For better performance, "
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
