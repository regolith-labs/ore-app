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
            Hero{}
            Block {
                title: "Proof of work.",
                title2: "On Solana.",
                detail: "Ore introduces a novel proof-of-work protocol designed specifically for token distribution. It relies on Solana for consensus and speedy hash validation.",
                dark: true,
                cta: ("Learn more", "https://github.com/hardhatchad/ore")
                // TODO Live hashes?
            }
            // Block {
            //     title: "Guaranteed rewards.",
            //     detail: "Ore is the only token in existence that guarantees every participating miner will earn rewards, regardless of how much hash power is active on the network.",
            //     dark: false
            //     // TODO Live reward rate
            // }
            Block {
                title: "Stable supply.",
                title2: "Predictable growth.",
                detail: "Ore has an algorithmic supply, programmed for constant linear growth. Every minute, one new Ore token is minted and distributed amongst active miners.",
                dark: false,
                cta: ("Learn more", "https://github.com/hardhatchad/ore")
                // TODO Live current supply
                // TODO Claimed vs mined
                // TODO Total holders
            }
            Block {
                title: "Fair launch.",
                title2: "Immutable code.",
                detail: "Ore has no insider allocation nor pre-mined supply. The smart contract has been open sourced and frozen to prevent tampering or removal.",
                dark: true,
                cta: ("Read the code", "https://github.com/hardhatchad/ore")
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
            Navbar {}
            div {
                class: "flex flex-col gap-y-8 sm:gap-y-10 md:gap-y-12 mx-auto my-auto pb-24",
                div {
                    class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8",
                    p {
                        class: "text-center text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold font-hero",
                        "It's time to mine."
                    }
                    p {
                        class: "text-lg sm:text-xl md:text-2xl lg:text-3xl text-center max-w-[44rem] font-hero leading-relaxed",
                        "Ore is the only cryptocurrency in the world that guarantees every miner can earn rewards."
                    }
                }
                Link {
                    // class: "mx-auto text-xl sm:text-2xl md:text-3xl lg:text-4xl font-semibold hover:bg-black hover:text-white transition-colors rounded-full px-6 py-3",
                    class: "mx-auto sm:text-lg md:text-xl lg:text-2xl font-semibold bg-green-500 hover:bg-green-600 active:bg-green-700 text-white transition-colors rounded-full px-6 py-3",
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
    cta: Option<(&'a str, &'a str)>,
    dark: bool,
) -> Element {
    let colors = if *dark {
        "bg-black text-white"
    } else {
        "bg-white text-black"
    };
    render! {
        div {
            class: "flex w-full h-svh z-20",
            div {
                class: "flex flex-col gap-4 sm:gap-6 md:gap-8 h-full w-full py-8 px-4 sm:px-8 {colors}",
                p {
                    class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                    "{title}"
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
                p {
                    class: "text-lg sm:text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-hero",
                    "{detail}"
                }
                if let Some((cta, to)) = cta {
                    render! {
                        Link {
                            class: "font-semibold mt-4",
                            to: "{to}",
                            "{cta} →"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Navbar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row justify-between px-4 sm:px-8 py-4 w-full z-50",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10",
                h1 {
                    class: "text-xl font-black my-auto w-min",
                    "ORE"
                }
            }
        }
    }
}
