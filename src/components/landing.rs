use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_types::Transfer;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{ActivityIndicator, Footer, OreIcon, OreLogoIcon},
    hooks::{
        use_is_onboarded, use_ore_supply, use_transfers, ActivityFilter, UiTokenAmountBalance,
    },
    route::Route,
    utils::asset_path,
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum TextColor {
    Black,
    White,
}

pub fn Landing() -> Element {
    let themes = [
        (asset_path("rock.png"), TextColor::Black),
        (asset_path("rock-2.jpg"), TextColor::White),
        (asset_path("rock-3.png"), TextColor::White),
        // (asset_path("rock-4.png"), TextColor::White),
    ];
    let mut i = use_signal(|| 0usize);
    let nav = navigator();
    let is_onboarded = use_is_onboarded();

    use_future(move || async move {
        loop {
            async_std::task::sleep(Duration::from_secs(8)).await;
            i.set(i.cloned().saturating_add(1));
        }
    });

    // If the user is already onboarded, redirect to home.
    if is_onboarded.read().0 {
        nav.replace(Route::Home {});
    }

    let len = themes.len();
    let text_color = themes[*i.read() % len].1;
    rsx! {
        div {
            class: "relative flex flex-col",
            for (index, theme) in themes.iter().enumerate() {
                BgImg {
                    visible: *i.read() % len == index,
                    bg_img: theme.0.clone(),
                    index
                }
            }
            Hero {
                text_color
            }
            Block {
                title: &"Proof of work.",
                title2: &"On Solana.",
                detail: &"ORE is designed to be mined on laptops, phones, and home computers. You don't need advanced hardware to get started and earn tokens.",
                section: Section::A,
                text_color
            }
            Block {
                title: &"Limited supply.",
                title2: &"Steady opportunity.",
                detail: &"ORE has a fixed total supply of 21m tokens. For the next 40 years, one new token will be mined every minute on average by miners around the globe.",
                section: Section::B,
                text_color
            }
            Block {
                title: &"Fair launch.",
                title2: &"Immutable code.",
                detail: &"ORE has no insider token allocation nor pre-mined supply. The smart contract has been frozen and open-sourced to prevent tampering or removal.",
                section: Section::C,
                text_color
            }
            // Footer {}
        }
    }
}

#[component]
fn BgImg(visible: bool, bg_img: String, index: usize) -> Element {
    // let visibility = if visible { "visible" } else { "hidden" };
    let visibility = if visible { "opacity-100" } else { "opacity-0" };
    rsx! {
        div {
            key: "{index}",
            class: "fixed top-0 w-full h-full bg-cover bg-center transition-opacity duration-1000 z-0 {visibility}",
            style: "background-image: url({bg_img})"
        }
    }
}

#[component]
fn Navbar(text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    rsx! {
        div {
            class: "flex flex-row justify-between px-4 sm:px-8 py-8 w-full transition-colors duration-1000 {copy_color}",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10",
                OreLogoIcon {
                    class: "h-6 md:h-8"
                }
            }
            div {
                class: "sm:text-sm md:text-base lg:text-lg",
                // TODO Language translator
                // TODO Buy link to Jupiter
                // Link {
                //     class: "font-semibold text-white bg-black hover:bg-gray-900 active:bg-gray-800 transition-colors px-4 py-3 rounded-full",
                //     to: Route::Home {},
                //     "Get started →"
                // }
            }
        }
    }
}

#[component]
fn Hero(text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    let cta_color = match text_color {
        TextColor::Black => "bg-black text-white",
        TextColor::White => "bg-white text-black",
    };
    rsx! {
        div {
            class: "flex flex-col w-full h-screen z-50",
            Navbar {
                text_color
            }
            div {
                class: "flex flex-col gap-y-8 sm:gap-y-10 md:gap-y-12 w-full md:mx-auto my-auto pb-24 px-4 md:px-8",
                div {
                    class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8 {copy_color} transition-colors duration-1000",
                    p {
                        class: "text-left sm:text-center text-6xl md:text-7xl lg:text-8xl font-bold font-hero",
                        "It's time to mine."
                    }
                    p {
                        class: "text-left sm:text-center text-xl sm:text-2xl md:text-3xl lg:text-4xl font-hero font-medium w-full",
                        "ORE is a borderless digital currency everyone can mine."//" Start mining at home or on your phone today."
                    }
                }
                Link {
                    class: "mr-auto sm:mx-auto text-center sm:text-lg md:text-xl lg:text-2xl font-semibold hover:bg-gray-900 active:bg-gray-800 transition-colors duration-1000 {cta_color} px-6 py-3 rounded-full",
                    to: Route::Home {},
                    "Get started →"
                }
            }
        }
    }
}

#[component]
fn Block(
    title: String,
    title2: String,
    detail: String,
    section: Section,
    text_color: TextColor,
) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    let bg_img = match section {
        Section::A => asset_path("rock-4.jpg"),
        Section::B => asset_path("rock-8.png"),
        Section::C => asset_path("rock-2.png"),
    };
    rsx! {
        div {
            class: "flex w-full z-20 min-h-svh h-full",
            div {
                class: "flex flex-col h-full w-full py-16 gap-24 px-4 sm:px-8",
                div {
                    class: "flex flex-col gap-4 sm:gap-6 md:gap-8 transition-colors duration-1000 {copy_color}",
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                        "{title}"
                        br {}
                        span {
                            class: "opacity-50",
                            "{title2}"
                        }
                    }
                    p {
                        class: "text-lg sm:text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-medium font-hero",
                        "{detail}"
                    }
                    BlockCta {
                        section: section.clone(),
                        text_color
                    }
                }
                div {
                    class: "flex h-full w-full",
                    match section {
                        // Section::A => rsx! { SectionA {} },
                        Section::B => rsx! { SectionB { text_color } },
                        _ => None
                    }
                }
            }
        }
    }
}

#[component]
fn BlockCta(section: Section, text_color: TextColor) -> Element {
    let style = "font-semibold mt-4 rounded py-2 transition-colors duration-1000";
    let cta_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    match section {
        Section::A => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: Route::WhatIsMining {},
                "Learn more →"
            }
        },
        Section::B => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: Route::OreTokenomics {},
                "Learn more →"
            }
        },
        Section::C => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: "https://github.com/regolith-labs/ore",
                "Read the code →"
            }
        },
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Section {
    A,
    B,
    C,
}

fn SectionA() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full my-auto gap-4 max-w-[48rem]",
            div {
                class: "flex flex-row gap-2",
                ActivityIndicator {}
                p {
                    class: "font-semibold text-xl opacity-50",
                    "Live transactions"
                }
            }
            div {
                class: "flex flex-col w-full",
                TransfersSection {}
            }
        }
    }
}

fn TransfersSection() -> Element {
    let filter = use_signal(|| ActivityFilter::Global);
    let offset = use_signal(|| 0);
    let transfers = use_transfers(filter, offset);
    let e = if let Some(transfers) = transfers.read().clone() {
        match transfers {
            Ok(transfers) => {
                rsx! {
                    if transfers.data.is_empty() {
                        p {
                            class: "text-sm opacity-50",
                            "No transactions yet"
                        }
                    }
                    for (i, transfer) in transfers.data.iter().enumerate() {
                        if i.le(&5) {
                            SimpleTransferRow {
                                transfer: transfer.clone()
                            }
                        } else {
                            div {}
                        }
                    }
                }
            }
            _ => None,
        }
    } else {
        None
    };
    e
}

#[component]
fn SimpleTransferRow(transfer: Transfer) -> Element {
    let addr = transfer.to_address[..5].to_string();
    let amount = amount_to_ui_amount(transfer.amount as u64, ore::TOKEN_DECIMALS);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = Duration::from_secs(transfer.ts as u64);
    let time = now.saturating_sub(ts);
    let t = time.as_secs();
    const ONE_MIN: u64 = 60;
    const ONE_HOUR: u64 = ONE_MIN * 60;
    const ONE_DAY: u64 = ONE_HOUR * 24;
    let time_str = if t.gt(&ONE_DAY) {
        format!("{}d ago", t.saturating_div(ONE_DAY))
    } else if t.gt(&ONE_HOUR) {
        format!("{}h ago", t.saturating_div(ONE_HOUR))
    } else if t.gt(&ONE_MIN) {
        format!("{}m ago", t.saturating_div(ONE_MIN))
    } else {
        format!("{}s ago", t)
    };

    rsx! {
        div {
            class: "flex flex-row py-3 gap-3 w-full transition-colors rounded hover:bg-gray-900 px-2 -mx-2",
            div {
                class: "flex flex-col pt-1",
                p {
                    class: "flex flex-row gap-2",
                    span {
                        class: "font-mono font-bold",
                        "{addr}"
                    }
                    "mined "
                    span {
                        class: "flex flex-row font-semibold gap-0.5",
                        OreIcon {
                            class: "w-3.5 h-3.5 my-auto",
                        }
                        "{amount:.4}"
                    }
                }
            }
            div {
                class: "flex pt-1.5 ml-auto",
                p {
                    class: "opacity-50 text-right text-nowrap text-sm",
                    "{time_str}"
                }
            }
        }
    }
}

#[component]
fn SectionB(text_color: TextColor) -> Element {
    let supply = use_ore_supply();
    let circulating_supply = supply
        .cloned()
        .and_then(|s| s.ok())
        .map(|s| amount_to_ui_amount(s.balance(), s.decimals))
        .unwrap_or_else(|| 0f64) as u64;
    rsx! {
        div {
            class: "flex flex-col gap-12 my-auto",
            OreValue {
                title: "Current supply".to_string(),
                amount: circulating_supply,
                text_color
            }
            OreValue {
                title: "Total supply".to_string(),
                amount: 21_000_000,
                text_color
            }
        }
    }
}

#[component]
fn OreValue(title: String, amount: u64, text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    rsx! {
        div {
            class: "flex flex-col gap-3 {copy_color} transition-colors duration-1000",
            p {
                class: "opacity-50 text-sm font-medium",
                "{title}"
            }
            div {
                class: "flex flex-row gap-2",
                OreIcon {
                    class: "w-6 h-6 md:w-7 md:h-7 lg:w-8 lg:h-8 my-auto"
                }
                p {
                    class: "text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                    "{amount.to_formatted_string(&Locale::en)}"
                }
            }
        }

    }
}
