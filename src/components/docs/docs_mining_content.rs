use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

use super::*;

pub fn MiningContent() -> Element {
    rsx! {
        ContentSection {
            MiningIntro {}
            MiningGetStarted {}
            MiningHowItWorks {}
            MiningFaq {}
        }
    }
}

fn MiningIntro() -> Element {
    rsx! {
        span {
            span {
                class: "font-semibold",
                "Mining is the process by which energy can be converted into cryptocurrency. "
            }
            "It allows anyone to permissionlessly convert electricity into liquid financial capital."
        }
    }
}

fn MiningGetStarted() -> Element {
    rsx! {
        span {
            class: "font-semibold text-xl pt-8 pb-2",
            "Get started"
        }
        BulletPointList {
            BulletPoint {
                number: "1",
                span {
                    "Connect your Solana wallet in the upper righthand corner of the app. If you don't have a wallet yet, you can "
                    Link {
                        class: "text-elements-gold hover:underline",
                        new_tab: true,
                        to: "https://phantom.com",
                        "install Phantom"
                    }
                    " to set one up."
                }
            }
            BulletPoint {
                number: "2",
                span {
                    "After connecting, navigate to the " Link {
                        class: "text-elements-gold hover:underline",
                        to: Route::Mine {},
                        "mining page"
                    }, ", and click \"Start\". You will be asked to pay a small one-time registration fee to join the public mining pool."
                }
            }
            BulletPoint {
                number: "3",
                span {
                    "Once registered, your miner will begin submitting solutions to the pool! "

                    if cfg!(feature = "web") {
                        "Unfortunately, web browsers will throttle your miner. To get the most power out of your machine, " Link {
                            class: "text-elements-gold hover:underline",
                            to: Route::Download {},
                            "download"
                        }, " the ORE desktop app. "
                    } else {
                        "Use the plus and minus buttons to configure how many CPU cores you want to allocate to mining."
                    }
                }
            }
        }
    }
}

fn MiningHowItWorks() -> Element {
    rsx! {
        span {
            class: "font-semibold text-xl pt-8 pb-2",
            "How it works"
        }
        BulletPointList {
            BulletPoint {
                "Miners perform large calculations that irreversibly turn electric power into a mathematical solution and heat. These solutions serve as an unforgeable proof that some computation was performed and energy was expended. "
            }
            BulletPoint {
                "A smart contract can verify these proofs and use them to securely mint a token, rewarding miners for their work. For this reason, mining is also often referred to as \"proof-of-work.\""
            }
            BulletPoint {
                "The net result of this process is that miners have a means of converting available energy into liquid financial capital."
            }
        }
        span {
            class: "font-semibold text-xl pt-8 pb-2",
            "FAQ"
        }
    }
}

fn MiningFaq() -> Element {
    rsx! {
        Col {
            class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start",
            Col {
                class: "w-full h-min justify-start",
                FaqItem {
                    question: "Do I have to pay transaction fees?",
                    "No, miners only need to pay a small one-time registration fee to join the public mining pool. The pool operator pays the transaction fee of submitting the best solution to the network."
                }
                if cfg!(feature = "web") {
                    FaqItem {
                        question: "How much power does it use?",
                        "When mining through the browser, you can currently only use one CPU core. To get more power, you can download the ORE "
                        Link {
                            new_tab: true,
                            to: "https://beta.ore.supply/download",
                            span { class: "text-elements-gold hover:underline", "desktop app" }
                        }
                        ", access more cores, and utilize the full potential of your machine."
                    }
                }
                FaqItem {
                    question: "Can I mine with my GPU?",
                    "Currently, the ORE mining app can only utilize your CPU. Support for high performance GPU mining is currently in development.",
                }
                FaqItem {
                    question: "What hash function does ORE use?",
                    "ORE uses "
                    Link {
                        new_tab: true,
                        to: "https://beta.ore.supply/download",
                        span { class: "text-elements-gold hover:underline", "Drillx" }
                    }
                    ", a CPU-friendly hash function designed to make mining accessible to anyone with a standard home computer."
                }
            }
        }
    }
}
