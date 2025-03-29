use crate::components::{Col, PlusIcon, Row};
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
        title: "Mining Overview",
        subtitle: "Discover how mining works and its role in the ecosystem.",
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
                    class: "text-xl font-semibold text-elements-highEmphasis",
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
                class: "text-lg text-elements-midEmphasis pb-8",
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
            LabelText {
                text: "How do I start mining ORE?"
            }
            StartMiningBullets {}
            LabelText {
                text: "How does ORE mining work?"
            }
            BodyText {
                text: "A computer performs a large calculation, converting electricity into a mathematical solution and heat. This solution serves as proof that the computation was done correctly. Another program verifies the proof and uses it to mint new ORE tokens as a reward"
            }
            // HowMiningWorksBullets {}
            // Col {
            //     class: "py-8",
            //     img {
            //         class: "relative w-full h-full object-contain z-10 rounded-lg",
            //         src: asset!("/public/ore-emissions-curve.png")
            //     }
            // }
            LabelText {text: "FAQ"}
            Faq {}
        }
    }
}

fn SupplyContent() -> Element {
    rsx! {
        ContentSection {
            LabelText {
                text: "What Is ORE's Supply Curve?"
            }
            SupplyCurveBullets {}
            Col {
                class: "py-8",
                img {
                    class: "relative w-full h-full object-contain z-10 rounded-lg",
                    src: asset!("/public/ore-emissions-curve.png")
                }
            }
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
                    class: "justify-start py-8",
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
                class: "overflow-y-auto scrollbar-hide",
                style: "padding-bottom: 1rem;",
                match *current_tab.read() {
                    MineHelpTabs::Mining => rsx! { MiningContent {} },
                    MineHelpTabs::Supply => rsx! { SupplyContent {} },
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
                class: "overflow-y-auto scrollbar-hide",
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
                            e.stop_propagation();
                            on_close.call(e.clone());
                        },
                        Row {
                            class: "w-full justify-between items-center",
                            Row {
                                class: "items-center",
                                gap: 4,
                                Col {
                                    class: "my-4",
                                    img { class: "w-8 h-8 rounded-full shrink-0", src: "{token.token.image}" }
                                }
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
            Col {
                class: "w-full h-min justify-start",
                FaqItem {
                    question: "How much computer power does it use?",
                    answer: "When mining through the browser, you can only use 1 core. By downloading the ORE desktop application, you can choose how many cores to use, ranging from one to the maximum available on your device."
                }
                FaqItem {
                    question: "How do I claim my rewards?",
                    answer: "To claim your rewards, simply click Claim and then confirm the transaction."
                }
                FaqItem {
                    question: "Does it use my CPU or GPU?",
                    answer: "Currently, ORE mining on both the web and desktop app uses your CPU. We are planning to release a GPU implementation soon, which will be open-sourced."
                }
                FaqItem {
                    question: "What Hash Function Does ORE Use?",
                    answer: "ORE employs Drillx, a CPU-friendly hash function tailored for its mining process, ensuring accessibility for anyone with a standard home computer."
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
                class: "justify-between font-wide text-left text-md w-full text-elements-highEmphasis",
                gap: 8,
                "{question}"
                PlusIcon {
                    class: "w-4 h-4 my-auto shrink-0 transition-transform duration-300 ease-in-out text-elements-lowEmphasis {rotation}"
                }
            }
            div {
                class: "overflow-hidden transition-all duration-300 ease-in-out {answer_class}",
                p {
                    class: "text-elements-midEmphasis mt-4 text-left",
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
fn BulletPoint(title: Option<String>, description: String) -> Element {
    rsx! {
        Row {
            class: "items-start pl-2",
            // Bullet point
            span {
                class: "text-elements-highEmphasis mr-2 select-none",
                "•"
            }
            // Content
            span {
                class: "flex-1",
                // Title in white with colon
                if let Some(title_text) = &title {
                    span {
                        class: "font-semibold text-elements-highEmphasis",
                        "{title_text}: "
                    }
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
fn StartMiningBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4 pb-8",
            BulletPointList {
                BulletPoint {
                    title: None,
                    description: "Connect any supported Solana wallet"
                }
                BulletPoint {
                    title: None,
                    description: "Click \"Start\" to join a pool–you will pay a small one-time Solana transaction fee"
                }
                BulletPoint {
                    title: None,
                    description: "Once the fee is paid, your machine will start mining and earning rewards"
                }
            }
        }
    }
}

fn HowMiningWorksBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4",
            BulletPointList {
                BulletPoint {
                    title: Some("Conversion".to_string()),
                    description: "A computer performs a large calculation, converting electricity into a mathematical solution and heat"
                }
                BulletPoint {
                    title: Some("Proof".to_string()),
                    description: "This solution serves as proof that the computation was done correctly"
                }
                BulletPoint {
                    title: Some("Reward".to_string()),
                    description: "Another program verifies the proof and uses it to mint new ORE tokens as a reward"
                }
            }
        }
    }
}

// fn HowMiningWorksBullets() -> Element {
//     rsx! {
//         Col {
//             class: "w-full ml-4",
//             BulletPointList {
//                 BulletPoint {
//                     title: "Income",
//                     description: "A computer performs a large calculation, converting electricity into a mathematical solution and heat"
//                 }
//                 BulletPoint {
//                     title: "Farm",
//                     description: "This solution serves as proof that the computation was done correctly"
//                 }
//                 BulletPoint {
//                     title: "Liquidity",
//                     description: "Another program verifies the proof and uses it to mint new ORE tokens as a reward"
//                 }
//             }
//         }
//     }
// }

fn SupplyCurveBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4",
            BulletPointList {
                BulletPoint {
                    title: Some("Emissions".to_string()),
                    description: "Starts at 1 ORE per minute, with emissions reducing by 10% every 12 months"
                }
                BulletPoint {
                    title: Some("Limit".to_string()),
                    description: "Capped at a 5 million token hard limit"
                }
                BulletPoint {
                    title: Some("Duration".to_string()),
                    description: "Will take approximately 30 years to reach max supply"
                }
                BulletPoint {
                    title: Some("Deterministic".to_string()),
                    description: "The curve is deterministic — it remains the same regardless of the number of miners"
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
