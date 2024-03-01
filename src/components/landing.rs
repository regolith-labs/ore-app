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
            class: "fixed top-safe left-0 right-0 bottom-0 overflow-hidden bg-white",
            img {
                class: "object-cover h-full w-full",
                src: "/smoke-2.png"
            }

        }
        div {
            class: "flex flex-col",
            div {
                class: "sticky top-0 flex flex-col min-h-screen",
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
                    class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8 justify-center mx-auto my-auto pb-24",
                    p {
                        class: "text-center text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold",
                        "It's time to "
                        span {
                            class: "inline-block text-white leading-relaxed bg-green-500 px-2",
                            "mine."
                        }
                    }
                    Link {
                        class: "mx-auto text-xl sm:text-2xl md:text-3xl lg:text-4xl font-semibold hover:bg-black hover:text-white transition-colors rounded-full px-6 py-3",
                        to: Route::Home {},
                        "Get started →"
                    }
                }
            }
            div {
                class: "flex w-full h-dvh p-2 sm:p-4 md:p-8 z-20",
                div {
                    class: "flex flex-col gap-2 sm:gap-4 md:gap-6 bg-black text-white rounded-lg shadow-lg h-full w-full p-4 sm:p-8",
                    div {
                        class: "flex flex-col",
                        p {
                            class: "text-white text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold leading-[1.55]",
                            "Proof of work."
                            br {}
                            "On Solana."
                        }
                    }
                    p {
                        class: "text-white text-lg sm:text-xl md:text-2xl lg:text-3xl mt-8 leading-relaxed max-w-[48rem]",
                        "Ore is a cryptocurrency everyone can mine. Smaller miners are guaranteed to always earn rewards and can't get pushed out by larger players."
                    }
                }
            }
        }
            // div {
            //     class: "w-1/2 flex flex-col gap-12 p-16 pb-32",
            //     div {
            //         class: "flex flex-col gap-4",
            //         p {
            //             class: "text-5xl font-bold",
            //             "Guaranteed rewards."
            //         }
            //         p {
            //             class: "text-white text-3xl mt-8 leading-5",
            //             "Smaller miners are guaranteed to earn rewards and can't get pushed out by larger players."
            //         }
            //         // p {
            //         //     class: "text-3xl leading-7",
            //         //     "Ore's global token supply grows at a constant linear rate, regardless of how much hashpower is on the network."
            //         // }
            //     }
            // }
        // div {
        //     class: "sticky top-0 flex flex-row w-full min-h-screen z-20",
        //     div {
        //         class: "flex flex-col gap-4 bg-green-500 text-white w-1/2",
        //         p {
        //             class: "sticky top-0 text-5xl font-medium p-16",
        //             "How much will you mine?"
        //         }
        //         // p {
        //         //     class: "text-white text-3xl",
        //         //     "Every participating miner is guaranteed to earn rewards. Large miners cannot exclude smaller players from earning rewards."
        //         // }
        //     }
        //     div {
        //         class: "bg-white text-black w-1/2 flex flex-col gap-12 p-16 pb-32",
        //         div {
        //             class: "flex flex-col gap-4",
        //             p {
        //                 class: "text-7xl font-bold",
        //                 "Stable supply."
        //             }
        //             p {
        //                 class: "text-3xl leading-7",
        //                 "Ore's global token supply grows at a constant linear rate, regardless of how much hashpower is on the network."
        //             }
        //         }
        //         p {
        //             class: "flex flex-col text-3xl font-bold mt-8",
        //             "1 ORE"
        //             span {
        //                 class: "text-lg font-medium",
        //                 "per minute"
        //             }
        //         }
        //         p {
        //             class: "flex flex-col text-3xl font-bold mt-8",
        //             "60 ORE"
        //             span {
        //                 class: "text-lg font-medium",
        //                 "per hour"
        //             }
        //         }
        //         p {
        //             class: "flex flex-col text-3xl font-bold mt-8",
        //             "1,440 ORE"
        //             span {
        //                 class: "text-lg font-medium",
        //                 "per hour"
        //             }
        //         }
        //         p {
        //             class: "flex flex-col text-3xl font-bold mt-8",
        //             "525,600 ORE"
        //             span {
        //                 class: "text-lg font-medium",
        //                 "per year"
        //             }
        //         }
        //         p {
        //             class: "flex flex-col text-3xl font-bold mt-8",
        //             "5,256,000 ORE"
        //             span {
        //                 class: "text-lg font-medium",
        //                 "per decade"
        //             }
        //         }
            // }
        // }
    }
}
