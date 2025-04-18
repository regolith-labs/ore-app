use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

use super::*;

pub fn MiningContent() -> Element {
    rsx! {
        ContentSection {
            MiningIntro {}
            MiningHowItWorks {}
            MiningGetStarted {}
            MiningFaq {}
        }
    }
}

fn MiningIntro() -> Element {
    rsx! {
        span {
            span {
                class: "font-semibold text-elements-highEmphasis",
                "Mining is the process by which energy can be converted into cryptocurrency. "
            }
            "It allows anyone to permissionlessly convert electricity into liquid financial capital."
        }
    }
}

fn MiningGetStarted() -> Element {
    rsx! {
        SectionTitle {
            "Get started"
        }
        BulletPointList {
            BulletPoint {
                number: "1",
                span {
                    "Connect your Solana wallet in the upper righthand corner of the app. If you don't have a wallet yet, you can install "
                    Link {
                        class: "text-elements-gold hover:underline font-medium",
                        new_tab: true,
                        to: "https://phantom.com",
                        "Phantom"
                    }
                    " to set one up."
                }
            }
            BulletPoint {
                number: "2",
                span {
                    "After connecting, navigate to the " Link {
                        class: "text-elements-gold hover:underline font-medium",
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
                        "Unfortunately, web browsers will throttle your miner. To get the most power out of your machine, download the " Link {
                            class: "text-elements-gold hover:underline font-medium",
                            to: Route::Download {},
                            "ORE desktop app"
                        }, "."
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
        SectionTitle {
            "How it works"
        }
        BulletPointList {
            BulletPoint {
                "Miners perform large calculations that turn electric power into mathematical solutions and heat. These solutions serve as an unforgeable proof that some computation was performed and energy was expended. "
            }
            BulletPoint {
                "A smart contract can verify these solutions and use them to securely mint a token, rewarding miners for their work. For this reason, mining is also often referred to as \"proof-of-work.\""
            }
            BulletPoint {
                "The net result of this process is that it gives miners a means of using computers to convert available energy into cryptocurrency which can be used in decentralized finance."
            }
        }
    }
}

fn MiningFaq() -> Element {
    rsx! {
        SectionTitle {
            "FAQ"
        }
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
                        "When mining through the browser, you can currently use only one CPU core. To get more out of your machine, checkout the "
                        Link {
                            new_tab: true,
                            to: "https://ore.supply/download",
                            span { class: "text-elements-gold hover:underline font-medium", "ORE desktop app" }
                        }
                        ", which allows you to access more cores."
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
                        to: "https://github.com/regolith-labs/drillx",
                        span { class: "text-elements-gold hover:underline font-medium", "Drillx" }
                    }
                    ", a CPU-friendly hash function designed to make mining accessible to anyone with a standard home computer."
                }
            }
        }
    }
}
