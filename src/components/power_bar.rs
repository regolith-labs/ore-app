use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{
    components::WarningIcon,
    hooks::{use_power_level, PowerLevel},
    route::Route,
};

#[component]
pub fn MinerPower(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-8 mt-auto",
            PowerHeader {}
            PowerBar {}
            DownloadLink {}
        }
    }
}

#[component]
fn PowerHeader(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-2",
            h2 {
                class: "text-2xl text-white font-bold",
                "Power level"
            }
            p {
                class: "text-lg",
                "Select how much computing power to dedicate to mining."
            }
            // p {
            //     class: "text-sm text-white opacity-80",
            //     "Higher power levels can earn more rewards but may impact your computer's battery life."
            // }
        }
    }
}

#[component]
fn PowerBar(cx: Scope) -> Element {
    let power_level = use_power_level(cx);
    let hover_level = use_state::<Option<u8>>(cx, || None);
    render! {
        div {
            class: "flex flex-row gap-1 w-full",
            PowerBarLevel { id: 0, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 1, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 2, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 3, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 4, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 5, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 6, power_level: power_level, hover_level: hover_level }
            PowerBarLevel { id: 7, power_level: power_level, hover_level: hover_level }
        }
    }
}

#[component]
fn PowerBarLevel<'a>(
    cx: Scope,
    id: u8,
    power_level: &'a UseSharedState<PowerLevel>,
    hover_level: &'a UseState<Option<u8>>,
) -> Element {
    let rounded = if id.eq(&0) {
        "rounded-l-full"
    } else if id.eq(&7) {
        "rounded-r-full"
    } else {
        ""
    };
    let allowed = if cfg!(feature = "web") && id.gt(&0) {
        "hover:cursor-not-allowed"
    } else {
        ""
    };
    let opacity = if cfg!(feature = "web") {
        if id.eq(&0) {
            ""
        } else {
            "opacity-40"
        }
    } else if let Some(hover_level) = hover_level.get() {
        if id.le(&power_level.read().0) {
            if id.le(hover_level) {
                ""
            } else {
                "opacity-40"
            }
        } else if id.le(hover_level) {
            "opacity-50"
        } else {
            "opacity-40"
        }
    } else if id.le(&power_level.read().0) {
        ""
    } else {
        "opacity-40"
    };
    let title = if cfg!(feature = "web") {
        if id.eq(&0) {
            ""
        } else {
            "This power level is not suppported by your web browser."
        }
    } else {
        ""
    };
    render! {
        button {
            class: "w-full h-4 bg-white transition-opacity {rounded} {opacity} {allowed}",
            title: "{title}",
            onmouseover: move |_| {
                hover_level.set(Some(*id));
            },
            onmouseout: move |_| {
                hover_level.set(None);
            },
            onclick: move |_| {
                *power_level.write() = PowerLevel(*id);
            }
        }
    }
}

#[component]
fn DownloadLink(cx: Scope) -> Element {
    if cfg!(feature = "web") {
        render! {
            div {
                class: "flex flex-row gap-2",
                WarningIcon {
                    class: "w-4 h-4 my-auto",
                }
                p {
                    class: "text-sm text-nowrap my-auto",
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
