use crate::components::{Col, PlusIcon, Row};
use crate::config::Token;
use crate::gateway::{ore::OreGateway, ore::RewardData, Rpc, UiTokenAmount};
use crate::hooks::{use_gateway, HelpDrawerPage, HelpDrawerState};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum Align {
    Left,
    Center,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum MineHelpTabs {
    Mining,
    Supply,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum StakeHelpTabs {
    Boosts,
    Yield,
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
            ("Boosts", StakeHelpTabs::Boosts),
            ("Yield", StakeHelpTabs::Yield),
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

// Mining Components
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

fn MiningContent() -> Element {
    rsx! {
        ContentSection {
            // LabelText {
            //     text: "Mining"
            // }
            BodyText {
                text: "Mining is the process by which energy can be converted into cryptocurrency."
            }
            LabelText {
                text: "How do I start mining ORE?"
            }
            StartMiningBullets {}
            SuggestionText {}

            LabelText {
                text: "How does ORE mining work?"
            }
            BodyText {
                text: "A computer performs a large calculation, converting electricity into a mathematical solution and heat. Another program verifies the proof and uses it to mint new ORE tokens as a reward"
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

fn StartMiningBullets() -> Element {
    rsx! {
        Col {
           class: "w-full ml-4 mb-2",
            BulletPointList {
                BulletPoint {
                    title: None,
                    description: {
                        rsx! {
                            p {
                                class: "text-lg text-elements-midEmphasis text-left",
                                span {
                                    "Connect any supported Solana wallet"
                                }
                            }
                        }
                    }
                }
                BulletPoint {
                    title: None,
                    description: {
                        rsx! {
                            p {
                                class: "text-lg text-elements-midEmphasis text-left",
                                span {
                                    "Click \"start\" to begin mining"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn SupplyContent() -> Element {
    rsx! {
    ContentSection {
        LabelText {
            text: "What is ORE's supply curve?"
        }
        SupplyCurveBullets {}

        LabelText {
            text: "ORE Emission Curve"
        }
            img {
                class: "relative w-full h-full object-contain z-10 rounded-lg",
                src: asset!("/public/ore-emissions-curve.png")
            }
        }
    }
}

fn SupplyCurveBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4 pb-8",
            BulletPointList {
                BulletPoint {
                    title: Some("Emissions".to_string()),
                    description: rsx!("1 ORE per minute, with emissions reducing by 10% every 12 months")
                }
                BulletPoint {
                    title: Some("Limit".to_string()),
                    description: rsx!("Capped at a 5 million token hard limit")
                }
                BulletPoint {
                    title: Some("Duration".to_string()),
                    description: rsx!("Will take approximately 30 years to reach max supply")
                }
                BulletPoint {
                    title: Some("Deterministic".to_string()),
                    description: rsx!("The curve is deterministic — it remains the same regardless of the number of miners")
                }
            }
        }
    }
}

// Staking Components
#[component]
fn StakeHelpContent(on_close: EventHandler<MouseEvent>) -> Element {
    let mut current_tab = use_signal(|| StakeHelpTabs::Boosts);

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
                        "{HELP_TITLES.stake.title}"
                    }
                    span {
                        class: "text-lg text-elements-midEmphasis",
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
                    StakeHelpTabs::Boosts => rsx! { BoostsContent {} },
                    StakeHelpTabs::Yield => rsx! { YieldContent {} },
                }
            }
        }
    }
}

fn BoostsContent() -> Element {
    rsx! {
        ContentSection {
            LabelText {
                text: "Overview"
            }
            BoostsBullets {}
            LabelText {text: "FAQ"}
            StakeFaq {}
        }
    }
}

fn YieldContent() -> Element {
    let mut total_supply: Signal<Option<UiTokenAmount>> = use_signal(|| None);
    use_effect(move || {
        spawn(async move {
            if let Ok(data) = use_gateway().rpc.get_token_supply(&Token::ore().mint).await {
                total_supply.set(Some(data));
            }
        });
    });

    rsx! {
        ContentSection {
            TokenSupply {
                total_supply: total_supply.clone()
            }
            LabelText {
                text: "Yield Concepts"
            }
            YieldBullets {}
            LabelText {
                text: "Liquidity Incentives"
            }
            // TODDO: add subtitle for staking yields
            // ChartText {
            //     text: "Liquidity Incentives"
            // }
            Col {
                img {
                    class: "relative w-full h-full object-contain z-10 rounded-lg",
                    src: asset!("/public/liquidity-incentives.png")
                }
            }
            // TODO: ADDD SUBTEXTT TO CHARTS
            LabelText {
                text: "Rewards Rates"
            }
            RewardsData {}
        }
    }
}

fn YieldBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4 pb-8",
            BulletPointList {
                BulletPoint {
                    title: Some("Liquidity".to_string()),
                    description: rsx!("Liquidity providers stake eligible LP tokens with the ORE boost protocol")
                }
                BulletPoint {
                    title: Some("Yield".to_string()),
                    description: rsx!("When a miner submits a hash, they earn a boost which is then distributed to stakers as yield")
                }
                BulletPoint {
                    title: Some("Claim".to_string()),
                    description: rsx!("Stakers can claim their yield at any time, with no time lock on withdrawals")
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

fn StakeFaq() -> Element {
    rsx! {
        Col {
            class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start",
            Col {
                class: "w-full h-min justify-start",
                FaqItem {
                    question: "Will there be more boosts added?",
                    answer: "Yes, we plan to add boosts on strategic pairs to further strengthen the ORE liquidity network.",
                }
                FaqItem {
                    // TODO: FIX SPACING
                    question: "Why does the APY change?",
                    answer: "APY is calculated based on a 7-day rolling average. As more ORE is staked in a boost, the yield is split among more participants, causing the APY to decrease. Conversely, if the total staked amount goes down, the APY will rise. This is because a maximum of 1 ORE per minute is distributed between miners and stakers.",
                    answer_with_link: rsx! {
                        p {
                            class: "text-elements-midEmphasis mt-4 text-left",
                            span {
                                "When mining through the browser, you can only use 1 core. By downloading the ORE"
                            }
                            Link {
                                new_tab: true,
                                to: "https://beta.ore.supply/download",
                                span {
                                    class: "text-elements-gold hover:underline",
                                    "desktop application"
                                }
                            }
                            ", you can choose how many cores to use, ranging from one to the maximum available on your device."
                        }
                    }
                }
                FaqItem {
                    question: "What are the risks of providing liquidity?",
                    answer: "Providing liquidity comes with inherent financial risk, including but not limited to divergence loss. Divergence loss occurs when the relative price of deposited tokens changes, potentially reducing the value of the deposit compared to simply holding the tokens separately. Once deposited, your exposure to each token may shift depending on market movements.",
                }
            }
        }
    }
}

// Shared/Utility Components
#[component]
fn LabelText(text: String) -> Element {
    rsx! {
        Col {
            class: "w-full",
            div {
                class: "mb-4",
                span {
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

fn SuggestionText() -> Element {
    rsx! {
        Row {
            class: "pb-8",
            p {
                span {
                    class: "text-elements-lowEmphasis",
                    "If you do not have a wallet, download the phantom wallet "
                }
                Link {
                    new_tab: true,
                    to: "https://phantom.com/download",
                    span {
                        class: "text-elements-gold hover:underline text-sm",
                        "here"
                    }
                }
                span {
                    class: "text-elements-lowEmphasis text-sm",
                    " to create a wallet."
                }
            }
        }
    }
}

#[component]
fn TokenSupply(total_supply: Signal<Option<UiTokenAmount>>) -> Element {
    let mut token_supply: Signal<Option<UiTokenAmount>> = use_signal(|| None);

    use_effect(move || {
        spawn(async move {
            if let Ok(data) = use_gateway().rpc.get_token_supply(&Token::ore().mint).await {
                token_supply.set(Some(data));
            }
        });
    });

    rsx! {
        if let Some(supply) = token_supply.read().as_ref() {
            Row {
                class: "items-center pb-8",
                gap: 2,
                span {
                    class: "text-xl font-semibold text-elements-highEmphasis",
                    "Current Supply:"
                }
                Col {
                    class: "items-end justify-end",
                    span {
                        class: "text-elements-gold text-xl font-semibold",
                        "{supply.ui_amount_string}"
                    }
                }
            }
        }
    }
}

fn RewardsData() -> Element {
    let mut rewards_data: Signal<Option<Vec<RewardData>>> = use_signal(|| Some(Vec::new()));

    use_effect(move || {
        spawn(async move {
            if let Ok(data) = use_gateway().get_rewards_data().await {
                rewards_data.set(Some(data));
            } else {
                log::error!("Failed to fetch rewards data");
            }
        });
    });

    rsx! {
        div {
            match rewards_data.read().as_ref() {
                Some(rewards) if !rewards.is_empty() => {
                    // Split the rewards into two groups for two columns
                    let mid_point = (rewards.len() + 1) / 2; // Ceiling division to handle odd numbers
                    let (left_column, right_column) = rewards.split_at(mid_point);

                    rsx! {
                        Row {
                            class: "justify-between",
                            gap: 8,
                            // Left Column
                            Col {
                                class: "w-1/2 justify-start",
                                for reward in left_column {
                                    Row {
                                        class: "justify-between items-center my-1",
                                        gap: 4,
                                        span {
                                            class: "text-sm text-elements-midEmphasis",
                                            "{reward.key}:"
                                        }
                                        span {
                                            class: "text-sm text-elements-highEmphasis font-semibold",
                                            "{reward.value}"
                                        }
                                    }
                                }
                            }
                            // Right Column
                            Col {
                                class: "w-1/2 justify-start",
                                for reward in right_column {
                                    Row {
                                        class: "justify-between items-center my-1",
                                        gap: 4,
                                        span {
                                            class: "text-sm text-elements-midEmphasis",
                                            "{reward.key}:"
                                        }
                                        span {
                                            class: "text-sm text-elements-highEmphasis font-semibold",
                                            "{reward.value}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    div { "No rewards data available" }
                }
            }
        }
    }
}

#[component]
fn ChartText(text: String) -> Element {
    rsx! {
    Col {
        class: "items-center pb-8",
            span {
                class: "text-center text-lg text-elements-highEmphasis font-semibold",
                "{text}"
            }
        }
    }
}

fn Faq() -> Element {
    rsx! {
        Col {
            class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start",
            Col {
                class: "w-full h-min justify-start",
                FaqItem {
                    question: "Do I pay transaction fees to mine?",
                    answer_with_link: rsx! {
                        div {
                            class: "flex flex-wrap items-baseline text-left mt-4",
                            span {
                                class: "text-elements-midEmphasis",
                                "No, you only pay a small one time transaction fee to register with a "
                            }
                            Link {
                                new_tab: true,
                                to: "https://github.com/regolith-labs/ore-pool",
                                span {
                                    class: "text-elements-gold hover:underline inline",
                                    "mining pool."
                                }
                            }
                        }
                    }
                }
                FaqItem {
                    question: "How much computer power does it use?",
                    answer_with_link: rsx! {
                        p {
                            class: "text-elements-midEmphasis mt-4 text-left",
                            span {
                                "When mining through the browser, you can only use 1 core. By downloading the ORE "
                            }
                            Link {
                                class: "ml-1",
                                new_tab: true,
                                to: "https://beta.ore.supply/download",
                                span {
                                    class: "text-elements-gold hover:underline",
                                    " desktop application"
                                }
                            }
                            ", you can choose how many cores to use, ranging from one to the maximum available on your device."
                        }
                    }
                }
                FaqItem {
                    question: "Does it use my CPU or GPU?",
                    answer: "Currently, ORE mining on both the web and desktop app uses your CPU. GPU mining is currently in development.",
                    answer_with_link: None
                }
                FaqItem {
                    question: "What hash function does ORE use?",
                    // answer: "ORE employs Drillx, a CPU-friendly hash function tailored for its mining process, ensuring accessibility for anyone with a standard home computer.",
                    answer_with_link: rsx! {
                        p {
                            class: "text-elements-midEmphasis mt-4 text-left",
                            span {
                                "ORE employs "
                            }
                            Link {
                                new_tab: true,
                                to: "https://beta.ore.supply/download",
                                span {
                                    class: "text-elements-gold hover:underline",
                                    "Drillx"
                                }
                            }
                            ", a CPU-friendly hash function tailored for its mining process, ensuring accessibility for anyone with a standard home computer."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FaqItem(question: String, answer: Option<String>, answer_with_link: Option<Element>) -> Element {
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
                class: "justify-between text-left text-lg w-full text-elements-highEmphasis",
                gap: 8,
                "{question}"
                PlusIcon {
                    class: "w-4 h-4 my-auto shrink-0 transition-transform duration-300 ease-in-out text-elements-lowEmphasis {rotation}"
                }
            }
            if let Some(answer) = answer {
                div {
                    class: "overflow-hidden transition-all duration-300 ease-in-out {answer_class}",
                    p {
                        class: "text-elements-midEmphasis mt-4 text-left",
                        "{answer}"
                    }
                }
            }
            if let Some(answer_with_link) = answer_with_link {
                div {
                    class: "overflow-hidden transition-all duration-300 ease-in-out {answer_class}",
                    {answer_with_link}
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
fn BulletPoint(
    title: Option<String>,
    description: Option<Element>,
    description_with_link: Option<Element>,
) -> Element {
    rsx! {
        Row {
            class: "items-start pl-2",
            // Bullet point
            span {
                class: "text-elements-highEmphasis mr-2 select-none",
                "•"
            }
            // Content
            div {
                class: "flex-1",
                // Title in white with colon
                if let Some(title_text) = &title {
                    span {
                        class: "text-lg font-semibold text-elements-highEmphasis",
                        "{title_text}: "
                    }
                }
                if let Some(description_with_link) = description_with_link {
                    {description_with_link}
                }
                // Description in gray on the same line
                if let Some(description) = description {
                    span {
                        class: "text-lg text-elements-midEmphasis",
                        {description}
                    }
                }
            }
        }
    }
}

fn BoostsBullets() -> Element {
    rsx! {
        Col {
            class: "w-full ml-4 pb-8",
            BulletPointList {
                BulletPoint {
                    title: Some("Boosts".to_string()),
                    description: rsx!("ORE's native staking mechanism used to bootstrap liquidity")
                }
                BulletPoint {
                    title: Some("Yield".to_string()),
                    description: rsx!("A portion of all newly mined supply is distributed to liquidity providers as yield")
                }
                BulletPoint {
                    title: Some("Incentives".to_string()),
                    description: rsx!("These rewards help offset the intrinsic risks that liquidity providers take on when providing liquidity")
                }
                BulletPoint {
                    title: Some("Liquidity".to_string()),
                    description: rsx!("With these incentives, ORE ensures a deeply liquid market, which ensues the token is easily exchangeable")
                }
            }
        }
    }
}
