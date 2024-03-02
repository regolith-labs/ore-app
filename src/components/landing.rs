#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{hooks::use_is_onboarded, Route};

// TODO Hero
//      It's time to mine
//      Ore is a cryptocurrency everyone can mine
//      Get started
//
// TODO Proof of work. On Solana.
//      Ore solves the starvation problem...
//
// TODO Stable supply
//      Ore has a constant linear supply
//      1 ORE / min
//      60 ORE / hour
//      1440 ORE / day
//      525,600 ORE / year
//      Current supply: (LIVE number) activity indicator
//
// TODO Fair launch
//      The Ore program is frozen. There has been no pre-mining.
//
// TODO How much will you mine?
//
// TODO Footer
//      Twitter
//      Github
//      Why Ore?
//      What is proof-of-work?
//      What is Solana?

#[component]
pub fn Landing(cx: Scope) -> Element {
    let is_onboarded = use_is_onboarded(cx);
    let nav = use_navigator(cx);

    // If the user is already onboarded, redirect to home.
    if is_onboarded.read().0 {
        nav.replace(Route::Home {});
    }

    render! {
        div {
            class: "flex flex-col",
            Navbar {}
            Hero{}
            Block {
                title: "Proof of work.",
                title2: "On Solana.",
                detail: "Ore is a cryptocurrency everyone can mine.",
                dark: true
            }
            Block {
                title: "Guaranteed rewards.",
                detail: "Ore is the only token in existence that guarantees every participating miner will earn rewards, regardless of how much hash power is active on the network.",
                dark: false
            }
            Block {
                title: "Stable supply.",
                detail: "Ore has an algorithmic supply, programmed for constant linear growth. Every minute, one new Ore token on average will be minted and distributed amongst all active miners.",
                dark: true
                // TODO Live current supply
                // TODO Claimed vs mined
                // TODO Total holders
            }
            Block {
                title: "Fair launch.",
                detail: "Ore has no pre-mining, no insider allocation, and has received no venture investment. It is open source and deployed with a frozen contract on day 1 to prevent tampering or removal.",
                dark: false
                // TODO These should be blocks "No pre-mining." "No insider allocations" "Frozen contract." "Open source."
            }
        }
    }
}

#[component]
fn Hero(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col w-full h-screen z-20 bg-cover bg-center bg-opacity-80",
            style: "background-image: url(/smoke-2.png)",
            div {
                class: "flex flex-col gap-y-6 sm:gap-y-8 md:gap-y-12 mx-auto my-auto pb-24",
                div {
                    class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8",
                    p {
                        class: "text-center text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold",
                        "It's time to mine."
                        // span {
                        //     class: "inline-block text-white leading-relaxed bg-green-500 px-2",
                        //     "mine."
                        // }
                    }
                    p {
                        class: "text-xl sm:text-2xl md:text-3xl lg:text-4xl text-center",
                        "Ore is a cryptocurrency everyone can mine."
                    }
                }
                Link {
                    // class: "mx-auto text-xl sm:text-2xl md:text-3xl lg:text-4xl font-semibold hover:bg-black hover:text-white transition-colors rounded-full px-6 py-3",
                    class: "mx-auto text-lg sm:text-xl md:text-2xl lg:text-3xl font-semibold bg-green-500 hover:bg-green-600 active:bg-green-700 text-white transition-colors rounded-full px-6 py-3",
                    to: Route::Home {},
                    "Get started →"
                }
            }
        }
    }
}

#[component]
fn Block<'a>(
    cx: Scope,
    title: &'a str,
    title2: Option<&'a str>,
    detail: &'a str,
    dark: bool,
) -> Element {
    let colors = if *dark {
        "bg-black text-white"
    } else {
        "bg-white text-black"
    };
    render! {
        div {
            // class: "flex w-full h-dvh p-2 sm:p-4 md:p-8 z-20",
            class: "flex w-full h-dvh z-20",
            div {
                class: "flex flex-col gap-2 sm:gap-4 md:gap-6 h-full w-full p-4 sm:p-8 {colors}",
                div {
                    class: "flex flex-col",
                    p {
                        class: "text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold leading-[1.75]",
                        "{title}"
                        // "Proof of work."
                        // br {}
                        // "On Solana."
                        if let Some(title2) = title2 {
                            render! {
                                br{}
                                span {
                                    class: "opacity-50",
                                    "{title2}"
                                }
                            }
                        }
                    }
                }
                p {
                    class: "text-lg sm:text-xl md:text-2xl lg:text-3xl mt-8 leading-relaxed max-w-[48rem]",
                    "{detail}"
                }
            }
        }
    }
}

#[component]
fn Navbar(cx: Scope) -> Element {
    render! {
        div {
            class: "fixed top-0 flex flex-row justify-between px-4 sm:px-8 py-6 w-full z-50",
            Link {
                to: Route::Landing {},
                class: "flex h-10",
                h1 {
                    class: "text-xl font-black my-auto w-min",
                    "ORE"
                }
            }
        }
    }
}
