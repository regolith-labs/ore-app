use crate::{
    components::{Col, PlusIcon, Row, StakingContent, TokenomicsContent},
    hooks::HelpDrawerState,
};
use dioxus::prelude::*;

use super::docs_mining_content::MiningContent;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum DocsTab {
    Mining,
    Staking,
    Tokenomics,
}

#[component]
pub fn DocsDrawerWrapper(
    drawer_state: Signal<HelpDrawerState>,
    on_close: EventHandler<MouseEvent>,
    drawer_remount: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "fixed right-0 top-0 flex flex-col h-full w-screen overflow-y-scroll sm:w-[574px] elevated border-l border-gray-800 text-white z-50 transition-transform duration-300 ease-in-out transform translate-x-0",
            onclick: move |e| e.stop_propagation(),
            DocsContent { on_close: on_close.clone() }
        }
    }
}

#[component]
fn DocsCloseButton(on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
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
    }
}
#[component]
fn DocsContent(on_close: EventHandler<MouseEvent>) -> Element {
    let current_tab = use_signal(|| DocsTab::Mining);
    rsx! {
        Fragment {
            Col {
                class: "w-full py-8" ,
                gap: 8,
                DocsHeader { on_close: on_close.clone() }
                Row {
                    class: "w-full mb-4 bg-surface-elevated border-b border-gray-800",
                    DocsTabButton { current_tab: current_tab.clone(), tab: DocsTab::Mining }
                    DocsTabButton { current_tab: current_tab.clone(), tab: DocsTab::Staking }
                    DocsTabButton { current_tab: current_tab.clone(), tab: DocsTab::Tokenomics }
                }
                div {
                    match *current_tab.read() {
                        DocsTab::Mining => rsx! { MiningContent {} },
                        DocsTab::Staking => rsx! { StakingContent {} },
                        DocsTab::Tokenomics => rsx! { TokenomicsContent {} },
                    }
                }
            }
        }
    }
}

#[component]
fn DocsHeader(on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Row {
            class: "px-8 justify-between",
            Col {
                gap: 2,
                span {
                    class: "text-3xl font-semibold",
                    "Docs"
                }
                span {
                    class: "text-md text-elements-lowEmphasis",
                    "Learn more about ORE."
                }
            }
            DocsCloseButton {
                on_close: on_close.clone()
            }
        }

    }
}

#[component]
fn DocsTabButton(current_tab: Signal<DocsTab>, tab: DocsTab) -> Element {
    let title = match tab {
        DocsTab::Mining => "Mining",
        DocsTab::Staking => "Staking",
        DocsTab::Tokenomics => "Tokenomics",
    };
    rsx! {
        button {
            class: "flex-1 h-12 transition-colors font-semibold hover:cursor-pointer border-b",
            class: if *current_tab.read() == tab {
                "text-lg text-white border-controls-primary"
            } else {
                "text-lg text-elements-lowEmphasis"
            },
            onclick: move |_| current_tab.set(tab),
            "{title}"
        }
    }
}

// fn SupplyContent() -> Element {
//     rsx! {
//         ContentSection {
//             LabelText {
//                 text: "What is ORE's supply curve?"
//             }
//             SupplyCurveBullets {}
//             LabelText {
//                 text: "ORE Emission Curve"
//             }
//             BodyText {
//                 text: "The ORE emission curve is a function that determines the rate at which ORE is emitted into the network."
//             }
//             img {
//                 class: "relative w-full h-full object-contain z-10 rounded-lg",
//                 src: asset!("/public/ore-emissions-curve.webp"),
//                 loading: "eager",
//                 decoding: "sync",
//             }
//         }
//     }
// }

// fn SupplyCurveBullets() -> Element {
//     rsx! {
//         Col {
//             class: "w-full ml-4 pb-8",
//             BulletPointList {
//                 BulletPoint {
//                     title: Some("Emissions".to_string()),
//                     description: rsx!("1 ORE per minute, with emissions reducing by 10% every 12 months")
//                 }
//                 BulletPoint {
//                     title: Some("Limit".to_string()),
//                     description: rsx!("Capped at a 5 million token hard limit")
//                 }
//                 BulletPoint {
//                     title: Some("Duration".to_string()),
//                     description: rsx!("Will take approximately 30 years to reach max supply")
//                 }
//                 BulletPoint {
//                     title: Some("Deterministic".to_string()),
//                     description: rsx!("The curve is deterministic — it remains the same regardless of the number of miners")
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn StakeHelpContent(on_close: EventHandler<MouseEvent>) -> Element {
//     let mut current_tab = use_signal(|| StakeHelpTabs::Boosts);
//     let mut rewards_data: Signal<Option<Vec<RewardData>>> = use_signal(|| None);
//     let mut total_supply: Signal<Option<UiTokenAmount>> = use_signal(|| None);

//     use_effect(move || {
//         spawn(async move {
//             if let Ok(data) = use_gateway().get_rewards_data().await {
//                 rewards_data.set(Some(data));
//             } else {
//                 log::error!("Failed to fetch rewards data");
//             }

//             if let Ok(data) = use_gateway().rpc.get_token_supply(&Token::ore().mint).await {
//                 total_supply.set(Some(data));
//             }
//         });
//     });

//     rsx! {
//         Fragment {
//             Col {
//                 class: "px-8 pt-4 pb-2",
//                 button {
//                     class: "rounded-full text-center py-1 w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer",
//                     onclick: move |e| {
//                         e.stop_propagation();
//                         on_close.call(e);
//                     },
//                     span {
//                         class: "text-xl font-semibold",
//                         "×"
//                     }
//                 }
//                 Col {
//                     class: "justify-start py-8",
//                     gap: 4,
//                     span {
//                         class: "text-2xl font-semibold",
//                         "{HELP_TITLES.stake.title}"
//                     }
//                     span {
//                         class: "text-lg text-elements-midEmphasis",
//                         "{HELP_TITLES.stake.subtitle}"
//                     }
//                 }
//             }

//             Row {
//                 class: "w-full mb-4 bg-surface-elevated border-b border-gray-800",
//                 {HELP_TITLES.stake.tabs.iter().map(|(label, tab)| {
//                     rsx! {
//                         button {
//                             class: "flex-1 h-12 transition-colors font-semibold hover:cursor-pointer border-b",
//                             class: if *current_tab.read() == *tab {
//                                 "text-lg text-white border-controls-primary"
//                             } else {
//                                 "text-lg text-elements-lowEmphasis"
//                             },
//                             onclick: move |_| current_tab.set(*tab),
//                             "{label}"
//                         }
//                     }
//                 })}
//             }

//             div {
//                 class: "overflow-y-auto scrollbar-hide",
//                 style: "padding-bottom: 1rem;",
//                 match *current_tab.read() {
//                     StakeHelpTabs::Boosts => rsx! { BoostsContent {} },
//                     StakeHelpTabs::Yield => rsx! { YieldContent {
//                         rewards_data: rewards_data.clone(),
//                         total_supply: total_supply.clone()
//                     } },
//                 }
//             }
//         }
//     }
// }

// fn BoostsContent() -> Element {
//     rsx! {
//         ContentSection {
//             LabelText { text: "Overview" }
//             BoostsBullets {}
//             LabelText { text: "FAQ" }
//             StakeFaq {}
//         }
//     }
// }

// #[component]
// fn YieldContent(
//     rewards_data: Signal<Option<Vec<RewardData>>>,
//     total_supply: Signal<Option<UiTokenAmount>>,
// ) -> Element {
//     rsx! {
//         ContentSection {
//             TokenSupply {
//                 total_supply: total_supply.clone()
//             }
//             LabelText { text: "Yield Concepts" }
//             YieldBullets {}
//             LabelText { text: "Liquidity Incentives" }
//             BodyText {
//                 text: "Liquidity providers stake eligible LP tokens with the ORE boost protocol."
//             }
//             Col {
//                 img {
//                     class: "relative w-full h-full object-contain z-10 rounded-lg",
//                     src: asset!("/public/liquidity-incentives.webp"),
//                     loading: "eager",
//                     decoding: "sync",
//                 }
//             }
//             LabelText { text: "Rewards Rates" }
//             BodyText {
//                 text: "The reward rates are determined by the difficulty level achieved when a miner submits a hash. More hashpower increases the probability of landing a higher difficulty."
//             }
//             RewardsData { rewards_data: rewards_data.clone() }
//         }
//     }
// }

// fn YieldBullets() -> Element {
//     rsx! {
//         Col {
//             class: "w-full ml-4 pb-8",
//             BulletPointList {
//                 BulletPoint {
//                     title: Some("Liquidity".to_string()),
//                     description: rsx!("Liquidity providers stake eligible LP tokens with the ORE boost protocol")
//                 }
//                 BulletPoint {
//                     title: Some("Yield".to_string()),
//                     description: rsx!("When a miner submits a hash, they earn a boost which is then distributed to stakers as yield")
//                 }
//                 BulletPoint {
//                     title: Some("Claim".to_string()),
//                     description: rsx!("Stakers can claim their yield at any time, with no time lock on withdrawals")
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn StakingFAQContent() -> Element {
//     rsx! {
//         Col {
//             class: "w-full px-8",
//             div {
//                 class: "mb-4",
//                 LabelText {
//                     text: "What is staking?"
//                 }
//             }
//             div {
//                 class: "mb-4",
//                 h3 {
//                     class: "text-lg font-semibold mb-2",
//                     "How is yield calculated?"
//                 }
//                 p {
//                     class: "text-elements-lowEmphasis",
//                     "Yield is calculated based on your deposit amount, the current protocol usage, and the time your tokens have been staked."
//                 }
//             }
//             div {
//                 class: "mb-4",
//                 h3 {
//                     class: "text-lg font-semibold mb-2",
//                     "Can I unstake at any time?"
//                 }
//                 p {
//                     class: "text-elements-lowEmphasis",
//                     "Yes, you can unstake your tokens at any time. Unstaking will stop the yield generation for those tokens."
//                 }
//             }
//         }
//     }
// }

// fn StakeFaq() -> Element {
//     rsx! {
//         Col {
//             class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start",
//             Col {
//                 class: "w-full h-min justify-start",
//                 FaqItem {
//                     question: "Will there be more boosts added?",
//                     answer: "Yes, we plan to add boosts on strategic pairs to further strengthen the ORE liquidity network.",
//                 }
//                 FaqItem {
//                     question: "Why does the APY change?",
//                     answer: "APY is calculated based on a 7-day rolling average. As more ORE is staked in a boost, the yield is split among more participants, causing the APY to decrease. Conversely, if the total staked amount goes down, the APY will rise. This is because a maximum of 1 ORE per minute is distributed between miners and stakers.",
//                     answer_with_link: rsx! {
//                         p {
//                             class: "text-elements-midEmphasis mt-4 text-left",
//                             span {
//                                 "When mining through the browser, you can only use 1 core. By downloading the ORE "
//                             }
//                             Link {
//                                 new_tab: true,
//                                 to: "https://beta.ore.supply/download",
//                                 span {
//                                     class: "text-elements-gold hover:underline",
//                                     "desktop application"
//                                 }
//                             }
//                             ", you can choose how many cores to use, ranging from one to the maximum available on your device."
//                         }
//                     }
//                 }
//                 FaqItem {
//                     question: "What are the risks of providing liquidity?",
//                     answer: "Providing liquidity comes with inherent financial risk, including but not limited to divergence loss. Divergence loss occurs when the relative price of deposited tokens changes, potentially reducing the value of the deposit compared to simply holding the tokens separately. Once deposited, your exposure to each token may shift depending on market movements.",
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn LabelText(text: String) -> Element {
//     rsx! {
//         Col {
//             class: "w-full",
//             div {
//                 class: "mb-4",
//                 span {
//                     class: "text-xl font-semibold text-elements-highEmphasis",
//                     "{text}"
//                 }
//             }
//         }
//     }
// }

// fn SuggestionText() -> Element {
//     rsx! {
//         Row {
//             class: "pb-8",
//             p {
//                 span {
//                     class: "text-elements-lowEmphasis",
//                     "If you do not have a wallet, download the phantom wallet "
//                 }
//                 Link {
//                     new_tab: true,
//                     to: "https://phantom.com/download",
//                     span {
//                         class: "text-elements-gold hover:underline text-sm",
//                         "here"
//                     }
//                 }
//                 span {
//                     class: "text-elements-lowEmphasis text-sm",
//                     " to create a wallet."
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn TokenSupply(total_supply: Signal<Option<UiTokenAmount>>) -> Element {
//     rsx! {
//         Row {
//             class: "items-center pb-8",
//             gap: 2,
//             span {
//                 class: "text-xl font-semibold text-elements-highEmphasis",
//                 "Current Supply:"
//             }
//             Col {
//                 class: "items-end justify-end",
//                 match total_supply.read().as_ref() {
//                     Some(supply) => {
//                         let amount_value = supply.ui_amount.unwrap_or(0.0);
//                         let formatted_supply = format_abbreviated_number(amount_value);
//                         rsx! {
//                             span {
//                                 class: "text-elements-gold text-xl font-semibold",
//                                 "{formatted_supply}"
//                             }
//                         }
//                     },
//                     None => rsx! {
//                         span {
//                             class: "w-16 h-8 rounded my-auto loading",
//                             ""
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn RewardsData(rewards_data: Signal<Option<Vec<RewardData>>>) -> Element {
//     rsx! {
//         div {
//             match rewards_data.read().as_ref() {
//                 Some(rewards) if !rewards.is_empty() => {
//                     let mid_point = (rewards.len() + 1) / 2;
//                     let (left_column, right_column) = rewards.split_at(mid_point);
//                     rsx! {
//                         Row {
//                             class: "justify-between",
//                             gap: 8,
//                             Col {
//                                 class: "w-1/2 justify-start",
//                                 for reward in left_column {
//                                     Row {
//                                         class: "justify-between items-center my-1",
//                                         gap: 4,
//                                         span {
//                                             class: "text-sm text-elements-midEmphasis",
//                                             "{reward.key}:"
//                                         }
//                                         span {
//                                             class: "text-sm text-elements-highEmphasis font-semibold",
//                                             "{reward.value}"
//                                         }
//                                     }
//                                 }
//                             }
//                             Col {
//                                 class: "w-1/2 justify-start",
//                                 for reward in right_column {
//                                     Row {
//                                         class: "justify-between items-center my-1",
//                                         gap: 4,
//                                         span {
//                                             class: "text-sm text-elements-midEmphasis",
//                                             "{reward.key}:"
//                                         }
//                                         span {
//                                             class: "text-sm text-elements-highEmphasis font-semibold",
//                                             "{reward.value}"
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 },
//                 Some(_) => rsx! {},
//                 None => rsx! {
//                     div {
//                         class: "flex justify-center items-center py-4",
//                         span {
//                             class: "text-elements-midEmphasis",
//                             "Loading rewards data..."
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn ChartText(text: String) -> Element {
//     rsx! {
//     Col {
//         class: "items-center pb-8",
//             span {
//                 class: "text-center text-lg text-elements-highEmphasis font-semibold",
//                 "{text}"
//             }
//         }
//     }
// }

// fn BoostsBullets() -> Element {
//     rsx! {
//         Col {
//             class: "w-full ml-4 pb-8",
//             BulletPointList {
//                 BulletPoint {
//                     title: Some("Boosts".to_string()),
//                     description: rsx!("ORE's native staking mechanism used to bootstrap liquidity")
//                 }
//                 BulletPoint {
//                     title: Some("Yield".to_string()),
//                     description: rsx!("A portion of all newly mined supply is distributed to liquidity providers as yield")
//                 }
//                 BulletPoint {
//                     title: Some("Incentives".to_string()),
//                     description: rsx!("These rewards help offset the intrinsic risks that liquidity providers take on when providing liquidity")
//                 }
//                 BulletPoint {
//                     title: Some("Liquidity".to_string()),
//                     description: rsx!("With these incentives, ORE ensures a deeply liquid market, which ensues the token is easily exchangeable")
//                 }
//             }
//         }
//     }
// }
