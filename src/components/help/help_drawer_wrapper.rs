use crate::components::{Col, CopyIcon, GlobeIcon, PaperAirplaneIcon, PlusIcon, Row};
use crate::hooks::{HelpDrawerPage, HelpDrawerState};
use dioxus::prelude::*;

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
            class: "fixed right-0 top-0 flex flex-col h-full w-screen sm:w-[574px] elevated elevated-border text-white z-50 transition-transform duration-300 ease-in-out transform translate-x-0",
            onclick: move |e| e.stop_propagation(),
            match current_page {
                HelpDrawerPage::Mine => rsx! { MineHelpContent { on_close: on_close.clone() } },
                HelpDrawerPage::Stake => rsx! { StakeHelpContent { on_close: on_close.clone() } },
            }
        }
    }
}

#[component]
fn LabelText(text: String) -> Element {
    rsx! {
        Col {
            class: "w-full",
            div {
                class: "mb-4",
                h3 {
                    class: "text-xl font-semibold text-elements-highEmphasis h-8",
                    "{text}"
                }
            }
        }
    }
}

#[component]
fn BodyText(text: String) -> Element {
    rsx! {
        span {
            class: "text-elements-midEmphasis mb-4",
            "{text}"
        }
    }
}

#[component]
fn ContentSection(children: Element) -> Element {
    rsx! {
        Col {
            class: "w-full px-8 py-8 scrollbar-hide",
            Col {
                // gap: 8,
                {children}
            }
        }
    }
}

fn MiningContent() -> Element {
    rsx! {
        ContentSection {
            LabelText {
                text: "What is mining?"
            }
            BodyText {
                text: "Mining is the process by which energy can be converted into cryptocurrency. It works by having a computer perform a large calculation that irreversibly turns electric power into a mathematical solution and heat."
            }
            MineBullets {}
            img {
                class: "relative w-full h-full object-contain z-10 rounded-lg my-8",
                src: asset!("/public/ore-emissions-curve.png")
            }
            LabelText {text: "FAQ"}
            Faq {}
        }
    }
}

#[component]
fn MiningGuideContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-8 py-8",
            gap: 4,
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
                img {
                    class: "relative w-full h-full pb-8 pt-8 object-contain z-10 rounded-lg",
                    src: asset!("/public/ore-emissions-curve.png")
                }
                h4 {
                    class: "text-lg font-semibold mb-2",
                    "How to Mine"
                }
                p {
                    class: "text-elements-lowEmphasis",
                    "Click the Start button to begin mining. You can adjust the number of CPU cores to allocate for mining."
                }
            }
            Faq {}
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
                class: "px-8 pt-4 pb-2",
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
                        class: "text-2xl font-semibold",
                        "{HELP_TITLES.mine.title}"
                    }
                    span {
                        class: "text-lg text-elements-midEmphasis",
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
                                "text-lg text-white border-controls-primary"
                            } else {
                                "text-lg text-elements-lowEmphasis"
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
                    MineHelpTabs::Mining => rsx! { MiningContent {} },
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
            class: "w-full px-8 py-7",
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
            class: "w-full px-8",
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
fn MiningSupplyContent() -> Element {
    rsx! {
        Col {
            class: "w-full px-8",
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
            class: "w-full px-8",
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
            class: "w-full px-8",
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
            class: "w-full px-8",
            div {
                class: "mb-4",
                LabelText {
                    text: "What is staking?"
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

#[derive(Clone, PartialEq)]
enum Align {
    Left,
    Center,
}
fn Faq() -> Element {
    rsx! {
        Col {
            class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start my-8",
            // SectionCopy {
            //     class: "text-left md:min-w-sm lg:min-w-md",
            //     align: Align::Left,
            //     tip: "Support",
            //     title: "FAQ",
            //     detail: "Commonly asked questions."
            // }
            Col {
                class: "w-full h-min justify-start",
                FaqItem {
                    question: "What is ORE?",
                    answer: "ORE is a new \"digital gold\" primitive for decentralized finance. It is a crypto commodity mineable via proof-of-work on the Solana blockchain."
                }
                FaqItem {
                    question: "Why should I care?",
                    answer: "ORE represents a new generation of digital gold, built for the new generation of crypto users. It takes the core properties of Bitcoin – fair launch, fixed supply, proof-of-work, immutability – and brings them to a new token on the Solana blockchain. "
                }
                FaqItem {
                    question: "How does mining work?",
                    answer: "ORE can be mined by anyone with a laptop or home computer. Simply navigate to the mining page of the app, connect your Solana wallet, and click the \"Start\" button. You will automatically be enrolled in a mining pool and do not need to pay any transaction fees while you mine."
                }
                FaqItem {
                    question: "How does liquidity work?",
                    answer: "ORE automatically distributes a portion of all newly mined supply to liquidity providers as yield. These incentives help bootstrap liquidity and maintain active markets with a network of assets in the Solana ecosystem. By providing liquidity for ORE pairs, stakers can earn fees from traders as well as receive additional rewards in the form of ORE yield."
                }
                FaqItem {
                    question: "Is it secure?",
                    answer: "ORE has been thoroughly audited by two independent auditing firms. The code is open source and has been battled tested in production. The development team is committed to freezing the protocol in the coming months to guarantee longterm security."
                }
            }
        }
    }
}

#[component]
fn FaqItem(question: String, answer: String) -> Element {
    let mut is_open = use_signal(|| false);
    let rotation = if is_open.cloned() {
        "rotate-45"
    } else {
        "rotate-0"
    };
    let answer_class = if is_open.cloned() {
        "max-h-96 opacity-100"
    } else {
        "max-h-0 opacity-0"
    };
    rsx! {
        button {
            class: "flex flex-col w-full py-4 px-2 sm:px-4 cursor-pointer transition-all duration-300 ease-in-out rounded-md hover:bg-elements-midEmphasis/10",
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "justify-between font-wide text-left font-bold text-md w-full text-elements-highEmphasis",
                gap: 8,
                "{question}"
                PlusIcon {
                    class: "w-4 h-4 my-auto shrink-0 transition-transform duration-300 ease-in-out text-elements-lowEmphasis {rotation}"
                }
            }
            div {
                class: "overflow-hidden transition-all duration-300 ease-in-out {answer_class}",
                p {
                    class: "text-elements-midEmphasis mt-4 text-lg text-left",
                    "{answer}"
                }
            }
        }
    }
}

#[component]
fn SectionCopy(
    class: Option<String>,
    align: Option<Align>,
    tip: Option<String>,
    title: String,
    subtitle: Option<String>,
    detail: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    let (text_align, text_margin) = match align.unwrap_or(Align::Center) {
        Align::Left => ("text-left", "mr-auto"),
        Align::Center => ("text-center", "mx-auto"),
    };
    rsx! {
        Col {
            class: "py-8 font-wide font-bold text-4xl md:text-5xl lg:text-6xl text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black px-4 {class} {text_align}",
            gap: 2,
            if let Some(tip) = tip {
                span {
                    // class: "z-30 text-elements-gold rounded-full w-min text-sm font-semibold mb-4 text-nowrap {text_margin}",
                    class: "z-30 border-2 border-elements-gold text-elements-gold rounded-full w-min px-3 py-1 text-xs font-semibold mb-4 text-nowrap {text_margin}",
                    "{tip}"
                }
            }
            span {
                class: "z-30",
                "{title}"
            }
            if let Some(subtitle) = subtitle {
                span {
                    class: "z-20 text-elements-lowEmphasis",
                    "{subtitle}"
                }
            }
            if let Some(detail) = detail {
                span {
                    class: "md:mb-auto mt-4 z-10 text-elements-midEmphasis font-wide font-medium text-lg sm:text-xl md:text-2xl {text_margin}",
                    "{detail}"
                }
            }
        }
    }
}

#[component]
fn BulletPointList(children: Element) -> Element {
    rsx! {
        Col {
            class: "w-full",
            gap: 2,
            {children}
        }
    }
}

#[component]
fn BulletPoint(title: String, description: String) -> Element {
    rsx! {
        Row {
            class: "items-start",
            // Bullet point
            span {
                class: "text-elements-highEmphasis mr-2 select-none",
                "•"
            }
            // Content
            span {
                class: "flex-1",
                // Title in white with colon
                span {
                    class: "font-semibold text-elements-highEmphasis",
                    "{title}: "
                }
                // Description in gray on the same line
                span {
                    class: "text-elements-midEmphasis",
                    "{description}"
                }
            }
        }
    }
}

// Example usage with the content from the image:
fn MineBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4",
            BulletPointList {
                BulletPoint {
                    title: "Income",
                    description: "Exchange variable yields for fixed yield tokens and earn predictable returns at maturity."
                }
                BulletPoint {
                    title: "Farm",
                    description: "Get leveraged exposure to variable yields and protocol points by buying yield tokens at an Implied APY."
                }
                BulletPoint {
                    title: "Liquidity",
                    description: "Earn extra yield on your productive assets by supplying them into Liquidity Vaults."
                }
            }
        }
    }
}

// #[component]
// fn BulletPoint(title: String, description: String) -> Element {
//     rsx! {
//         div {
//             class: "mb-6",
//             span {
//                 class: "text-lg font-semibold text-elements-highEmphasis block mb-1",
//                 "{title}"
//             }
//             span {
//                 class: "text-elements-midEmphasis",
//                 "{description}"
//             }
//         }
//     }
// }

// #[component]
// fn BulletPointList(children: Element) -> Element {
//     rsx! {
//         Col {
//             class: "w-full space-y-2",
//             {children}
//         }
//     }
// }

// // Example usage:
// fn MineBullets() -> Element {
//     rsx! {
//         Col {
//             BulletPointList {
//                 BulletPoint {
//                     title: "Connect",
//                     description: "Connect any supported Solana wallet"
//                 }
//                 BulletPoint {
//                     title: "Start",
//                     description: "Click "Start" to join a pool–you will pay a small one-time Solana transaction fee"
//                 }
//                 BulletPoint {
//                     title: "Earn",
//                     description: "Once the fee is paid, your machine will start mining and earning rewards"
//                 }
//             }
//         }
//     }
// }
