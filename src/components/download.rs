#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::CodeBlock;

pub fn Download() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            div {
                class: "flex flex-col gap-4",
                p {
                    class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                    "Download"
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
