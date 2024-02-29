#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{hooks::use_is_onboarded, Route};

#[component]
pub fn Landing(cx: Scope) -> Element {
    let is_onboarded = use_is_onboarded(cx);
    let nav = use_navigator(cx);

    // If the user is already onboarded, redirect to home.
    if is_onboarded.get() {
        nav.replace(Route::Home {});
    }

    render! {
        img {
            class: "fixed top-0 left-0 w-screen -z-50",
            src: "/smoke.png"
        }
        div {
            class: "sticky top-0 flex flex-col min-h-screen z-0",
            div {
                class: "flex flex-row justify-between px-4 sm:px-8 py-6 w-full",
                Link {
                    to: Route::Landing {},
                    class: "flex h-10",
                    h1 {
                        class: "text-xl font-black my-auto w-min",
                        "ORE"
                    }
                }
            }
            div {
                class: "flex flex-col gap-y-8 justify-center mx-auto my-auto",
                p {
                    class: "text-center text-7xl font-bold",
                    "It's time to "
                    span {
                        class: "inline-block text-white leading-relaxed bg-green-500 px-2",
                        "mine."
                    }
                }
                Link {
                    class: "mx-auto text-3xl font-semibold hover:underline",
                    to: Route::Home {},
                    "Start →"
                }
            }
        }
        div {
            class: "sticky top-0 bg-[#000] p-16 flex flex-col w-full min-h-screen z-10",
            div {
                class: "flex flex-col gap-4",
                p {
                    class: "text-white text-7xl font-bold",
                    "Proof of work."
                }
                p {
                    class: "text-white text-7xl font-bold",
                    "On Solana."
                }
            }
            p {
                class: "text-white text-3xl mt-8 w-1/2 leading-5",
                "Ore is a cryptocurrency everyone can mine. Smaller miners are guaranteed to earn rewards and can't get pushed out by bigger players."
            }
        }
        div {
            class: "sticky top-0 flex flex-row w-full min-h-screen z-20",
            div {
                class: "flex flex-col gap-4 bg-green-500 text-white w-1/2",
                p {
                    class: "sticky top-0 text-5xl font-medium p-16",
                    "How much will you mine?"
                }
                // p {
                //     class: "text-white text-3xl",
                //     "Every participating miner is guaranteed to earn rewards. Large miners cannot exclude smaller players from earning rewards."
                // }
            }
            div {
                class: "bg-white text-black w-1/2 flex flex-col gap-12 p-16 pb-32",
                div {
                    class: "flex flex-col gap-4",
                    p {
                        class: "text-7xl font-bold",
                        "Stable supply."
                    }
                    p {
                        class: "text-3xl leading-7",
                        "Ore's global token supply grows at a constant linear rate, regardless of how much hashpower is on the network."
                    }
                }
                p {
                    class: "flex flex-col text-3xl font-bold mt-8",
                    "1 ORE"
                    span {
                        class: "text-lg font-medium",
                        "per minute"
                    }
                }
                p {
                    class: "flex flex-col text-3xl font-bold mt-8",
                    "60 ORE"
                    span {
                        class: "text-lg font-medium",
                        "per hour"
                    }
                }
                p {
                    class: "flex flex-col text-3xl font-bold mt-8",
                    "1,440 ORE"
                    span {
                        class: "text-lg font-medium",
                        "per hour"
                    }
                }
                p {
                    class: "flex flex-col text-3xl font-bold mt-8",
                    "525,600 ORE"
                    span {
                        class: "text-lg font-medium",
                        "per year"
                    }
                }
                p {
                    class: "flex flex-col text-3xl font-bold mt-8",
                    "5,256,000 ORE"
                    span {
                        class: "text-lg font-medium",
                        "per decade"
                    }
                }
            }
        }
    }
}
