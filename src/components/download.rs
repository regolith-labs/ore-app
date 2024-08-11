#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{components::CodeBlock, route::Route};

pub fn Download() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            div {
                class: "flex flex-col gap-4",
                p {
                    class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                    "Clients"
                }
                p {
                    class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                    "App"
                }
                p {
                    "The ORE web app is the easiest way to get started and begin mining. Simply connect your Solana wallet in the app and click \"Mine\"."
                }
                Link {
                    class: "flex shrink font-semibold text-center mr-auto mt-4 px-5 py-3 bg-green-500 hover:bg-green-600 text-white transition-colors rounded-full",
                    to: Route::Home {},
                    p {
                        class: "text-lg text-center font-semibold",
                        "Launch app →"
                    }
                }
                p {
                    class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                    "CLI"
                }
                p {
                    "The ORE CLI provides a command line interface that can run on any computer or server. "
                    "To get started, ensure you have Rust installed. "
                }
                CodeBlock {
                    text: "curl https://sh.rustup.rs -sSf | sh"
                }
                p {
                    "Next, install the Solana toolchain and create a Solana keypair if you haven't done so already. "
                }
                CodeBlock {
                    text: "sh -c \"$(curl -sSfL https://release.solana.com/v1.18.4/install)\"\nsolana-keygen new"
                }
                p {
                    "Now, install the ORE CLI."
                }
                CodeBlock {
                    text: "cargo install ore-cli"
                }
                p {
                    "The ORE CLI uses your existing Solana config and keypair by default. "
                    "Ensure you have enough SOL topped up on your keypair to pay for transaction fees. "
                    "To begin mining, use the mine command."
                }
                CodeBlock {
                    text: "ore mine",
                }
                p {
                    "Use the -h flag to pull up a help menu anywhere in the CLI."
                }
                CodeBlock {
                    text: "ore -h"
                }
            }
        }
    }
}
