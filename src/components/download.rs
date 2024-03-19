#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::CodeBlock;

const DESKTOP_DOWNLOAD_MAC: &str =
    "https://github.com/HardhatChad/ore-app/releases/download/0.0.1/Ore-MacOS.zip";

#[component]
pub fn Download(cx: Scope) -> Element {
    render! {
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
                    "Desktop"
                }
                p {
                    "Coming soon..."
                    // "Currently the only way to install the Ore miner desktop app is to compile the source code from scratch. "
                    // "You can do this by downloading the code from the Github repository and following the build process described in the README."
                }
                a {
                    class: "font-semibold hover:underline",
                    href: "{DESKTOP_DOWNLOAD_MAC}",
                    "Click here to download"
                }
                p {
                    class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                    "Build from scratch"
                }
                p {
                    "Coming soon..."
                    // "Currently the only way to install the Ore miner desktop app is to compile the source code from scratch. "
                    // "You can do this by downloading the code from the Github repository and following the build process described in the README."
                }
                p {
                    class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                    "CLI"
                }
                p {
                    "The Ore command line interface (CLI) is designed for miners who want to get the most out of their machine. "
                    "You can install it on any computer or server to run an Ore miner in the background. "
                    "To get started, ensure you have Rust and cargo installed. "
                }
                CodeBlock {
                    text: "curl https://sh.rustup.rs -sSf | sh"
                }
                p {
                    "Next, install the Solana CLI if you haven't done so already. "
                }
                CodeBlock {
                    text: "sh -c \"$(curl -sSfL https://release.solana.com/v1.18.4/install)\""
                }
                p {
                    "Now, install the Ore CLI."
                }
                CodeBlock {
                    text: "cargo install ore-cli"
                }
                p {
                    "The Ore CLI uses your default Solana CLI config and identity. "
                    "Ensure you have enough SOL topped up on this account to pay for transaction fees. "
                    "To begin mining, simply use the mine command."
                }
                CodeBlock {
                    text: "ore mine"
                }
            }
        }
    }
}
