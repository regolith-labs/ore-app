use crate::components::{Col, CopyIcon, GlobeIcon, PaperAirplaneIcon, Row};
use crate::hooks::{use_wallet, HelpDrawerPage, HelpDrawerState, Wallet};
use crate::route::Route;
use dioxus::document::eval;
use dioxus::prelude::*;
use std::str::FromStr;
use {wasm_bindgen_futures, web_sys};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum MineHelpTabs {
    Mining,
    Supply,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum StakeHelpTabs {
    Yield,
    Boosts,
}

struct HelpTitles {
    mine: MineTitles,
    stake: StakeTitles,
}

struct MineTitles {
    title: &'static str,
    subtitle: &'static str,
    tabs: [(&'static str, MineHelpTabs); 2],
}

struct StakeTitles {
    title: &'static str,
    subtitle: &'static str,
    tabs: [(&'static str, StakeHelpTabs); 2],
}

const HELP_TITLES: HelpTitles = HelpTitles {
    mine: MineTitles {
        title: "Mining",
        subtitle: "Learn more about mining and the details of the mining process.",
        tabs: [
            ("Mining", MineHelpTabs::Mining),
            ("Supply", MineHelpTabs::Supply),
        ],
    },
    stake: StakeTitles {
        title: "Stake",
        subtitle: "Learn more about yield and the details of the yield process.",
        tabs: [
            ("Yield", StakeHelpTabs::Yield),
            ("Boosts", StakeHelpTabs::Boosts),
        ],
    },
};

#[component]
pub fn HelpDrawerWrapper(
    drawer_state: Signal<HelpDrawerState>,
    on_close: EventHandler<MouseEvent>,
    drawer_remount: Signal<bool>,
) -> Element {
    let current_page = &drawer_state.read().current_page;

    rsx! {
        div {
            class: "flex flex-col h-full w-screen sm:w-[574px] elevated elevated-border text-white z-50 relative",
            onclick: move |e| e.stop_propagation(),
            match current_page {
                HelpDrawerPage::Mine => rsx! { MineHelpContent { on_close: on_close.clone() } },
                HelpDrawerPage::Stake => rsx! { StakeHelpContent { on_close: on_close.clone() } },
            }
        }
    }
}

#[component]
fn MineHelpContent(on_close: EventHandler<MouseEvent>) -> Element {
    let mut current_tab = use_signal(|| MineHelpTabs::Mining);

    rsx! {
        Fragment {
            // Header
            Col {
                class: "px-4 pt-4 pb-2",
                // Close button
                button {
                    class: "rounded-full text-center py-1 w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_close.call(e);
                    },
                    span {
                        class: "text-xl font-semibold",
                        "×"
                    }
                }
                // Title content
                Col {
                    class: "justify-start my-4",
                    gap: 4,
                    span {
                        class: "text-xl font-semibold",
                        "{HELP_TITLES.mine.title}"
                    }
                    span {
                        class: "text-sm text-elements-lowEmphasis",
                        "{HELP_TITLES.mine.subtitle}"
                    }
                }
            }

            // Tabs
            Row {
                class: "w-full mb-4 bg-surface-elevated border-b border-gray-800",
                {HELP_TITLES.mine.tabs.iter().map(|(label, tab)| {
                    rsx! {
                        button {
                            class: "flex-1 h-12 transition-colors font-semibold hover:cursor-pointer border-b",
                            class: if *current_tab.read() == *tab {
                                "text-white border-controls-primary"
                            } else {
                                "text-elements-lowEmphasis"
                            },
                            onclick: move |_| current_tab.set(*tab),
                            "{label}"
                        }
                    }
                })}
            }

            // Content
            div {
                class: "overflow-y-auto",
                style: "padding-bottom: 1rem;",
                match *current_tab.read() {
                    MineHelpTabs::Mining => rsx! { MiningGuideContent {} },
                    MineHelpTabs::Supply => rsx! { MiningFAQContent {} },
                }
            }
        }
    }
}

#[component]
fn StakeHelpContent(on_close: EventHandler<MouseEvent>) -> Element {
    let mut current_tab = use_signal(|| StakeHelpTabs::Yield);

    rsx! {
        Fragment {
            // Header
            Col {
                class: "px-4 pt-4 pb-2",
                // Close button
                button {
                    class: "rounded-full text-center py-1 w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_close.call(e);
                    },
                    span {
                        class: "text-xl font-semibold",
                        "×"
                    }
                }
                // Title content
                Col {
                    class: "justify-start my-4",
                    gap: 4,
                    span {
                        class: "text-xl font-semibold",
                        "{HELP_TITLES.stake.title}"
                    }
                    span {
                        class: "text-sm text-elements-lowEmphasis",
                        "{HELP_TITLES.stake.subtitle}"
                    }
                }
            }

            // Tabs
            Row {
                class: "w-full mb-4 bg-surface-elevated border-b border-gray-800",
                {HELP_TITLES.stake.tabs.iter().map(|(label, tab)| {
                    rsx! {
                        button {
                            class: "flex-1 h-12 transition-colors font-semibold hover:cursor-pointer border-b",
                            class: if *current_tab.read() == *tab {
                                "text-white border-controls-primary"
                            } else {
                                "text-elements-lowEmphasis"
                            },
                            onclick: move |_| current_tab.set(*tab),
                            "{label}"
                        }
                    }
                })}
            }

            // Content
            div {
                class: "overflow-y-auto",
                style: "padding-bottom: 1rem;",
                match *current_tab.read() {
                    StakeHelpTabs::Yield => rsx! { StakeYieldContent {} },
                    StakeHelpTabs::Boosts => rsx! { MiningFAQContent {} },
                }
            }
        }
    }
}

#[component]
fn StakeBoostsContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-xl font-semibold mb-2",
                    "What is mining?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "ORE mining converts your computer's processing power into cryptocurrency."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "How to Mine"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Click the Start button to begin mining. You can adjust the number of CPU cores to allocate for mining."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "Mining Rewards"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Your mining rewards will accumulate in your pending rewards and can be claimed once confirmed."
                }
            }
        }
    }
}

#[component]
fn StakeYieldContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-xl font-semibold mb-2",
                    "Mining Guide"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "ORE mining converts your computer's processing power into cryptocurrency."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "How to Mine"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Click the Start button to begin mining. You can adjust the number of CPU cores to allocate for mining."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "Mining Rewards"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Your mining rewards will accumulate in your pending rewards and can be claimed once confirmed."
                }
            }
        }
    }
}

// Mining Guide content component
#[component]
fn MiningGuideContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-xl font-semibold mb-2",
                    "What is mining?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Mining is the process by which energy can be converted into cryptocurrency. It works by having a computer perform a large calculation that irreversibly turns electric power into a mathematical solution and heat. The generated solution serves as an unforgeable proof that the computation was performed correctly and without error. Another computer program can then verify this proof and use it to securely mint a token rewarding its creator for their work. For this reason, this process is also often referred to as proof-of-work. "
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "How to Mine"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Click the Start button to begin mining. You can adjust the number of CPU cores to allocate for mining."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "Mining Rewards"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Your mining rewards will accumulate in your pending rewards and can be claimed once confirmed."
                }
            }
        }
    }
}

#[component]
fn MiningSupplyContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-xl font-semibold mb-2",
                    "Mining Guide"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "ORE mining converts your computer's processing power into cryptocurrency."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "How to Mine"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Click the Start button to begin mining. You can adjust the number of CPU cores to allocate for mining."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "Mining Rewards"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Your mining rewards will accumulate in your pending rewards and can be claimed once confirmed."
                }
            }
        }
    }
}

// Staking Guide content component
#[component]
fn StakingGuideContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-xl font-semibold mb-2",
                    "Staking Guide"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Staking allows you to earn yield by providing liquidity to the protocol."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "How to Stake"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Deposit your tokens to start earning yield. Your net deposits and yield will be displayed in the Account section."
                }
            }
            div {
                class: "mb-4",
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "Claiming Rewards"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Use the Claim button to collect your accumulated staking rewards."
                }
            }
        }
    }
}

// Token content component - reuses the existing token content
#[component]
fn TokenContent(on_close: EventHandler<MouseEvent>) -> Element {
    let tokens = crate::hooks::use_tokens_with_values();

    rsx! {
        Col {
            class: "w-full",
            {tokens.iter().map(|token| {
                let token_clone = token.clone();

                rsx! {
                    div {
                        key: "{token.token.mint}",
                        class: "w-full justify-between items-center py-4 px-4 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
                        onclick: move |e| {
                            // First close drawer
                            e.stop_propagation();
                            on_close.call(e.clone());

                            // // Then navigate
                            // navigator.push(Route::TransferWithToken {
                            //     token_ticker: token_clone.token.ticker.clone()
                            // });
                        },
                        Row {
                            class: "w-full justify-between items-center",
                            Row {
                                class: "items-center",
                                gap: 4,
                                img { class: "w-8 h-8 rounded-full shrink-0", src: "{token.token.image}" }
                                Col {
                                    span { class: "font-medium text-elements-highEmphasis", "{token.token.name}" }
                                    span { class: "font-medium text-xs text-elements-lowEmphasis",
                                        "{token.balance:.4} {token.token.ticker}"
                                    }
                                }
                            }
                            Col {
                                class: "items-end",
                                "${token.total_value:.2}"
                            }
                        }
                    }
                }
            })}
        }
    }
}

// Mining FAQ content component
#[component]
fn MiningFAQContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-lg font-semibold mb-2",
                    "What is ORE mining?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "ORE mining allows you to earn cryptocurrency by contributing your computer's processing power to the network."
                }
            }
            div {
                class: "mb-4",
                h3 {
                    class: "text-lg font-semibold mb-2",
                    "How many cores should I dedicate?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "This depends on your computer's capabilities. You can adjust the number of cores to find a balance between mining efficiency and system performance."
                }
            }
            div {
                class: "mb-4",
                h3 {
                    class: "text-lg font-semibold mb-2",
                    "When do I receive my mining rewards?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Mining rewards first appear as pending rewards. They become claimable after being processed by the mining pool, typically within a few hours."
                }
            }
        }
    }
}

// Staking FAQ content component
#[component]
fn StakingFAQContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-4",
            div {
                class: "mb-4",
                h3 {
                    class: "text-lg font-semibold mb-2",
                    "What is staking?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Staking is the process of depositing your tokens to provide liquidity to the protocol and earn yield in return."
                }
            }
            div {
                class: "mb-4",
                h3 {
                    class: "text-lg font-semibold mb-2",
                    "How is yield calculated?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Yield is calculated based on your deposit amount, the current protocol usage, and the time your tokens have been staked."
                }
            }
            div {
                class: "mb-4",
                h3 {
                    class: "text-lg font-semibold mb-2",
                    "Can I unstake at any time?"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Yes, you can unstake your tokens at any time. Unstaking will stop the yield generation for those tokens."
                }
            }
        }
    }
}
