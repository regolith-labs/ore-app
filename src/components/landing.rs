use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_types::Transfer;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{ActivityIndicator, DiscordIcon, Footer, GithubIcon, OreIcon, OreLogoIcon, XIcon},
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
    // let mut current_page = use_signal(|| 0);
    let nav = navigator();
    let is_onboarded = use_is_onboarded();
    let mut i = use_signal(|| 0usize);
    let themes = [
        (asset_path("rock.png"), TextColor::Black),
        (asset_path("rock-2.jpg"), TextColor::White),
        (asset_path("rock-3.png"), TextColor::White),
        (asset_path("rock-4.png"), TextColor::White),
        // (asset_path("rock-5.jpg"), TextColor::White),
    ];
    let len = themes.len();
    let text_color = themes[*i.read() % len].1;

    // Change the background image every 8 sec
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

    rsx! {
        for (index, theme) in themes.iter().enumerate() {
            BgImg {
                visible: *i.read() % len == index,
                bg_img: theme.0.clone(),
                index
            }
        }
        div {
            class: "absolute top-0 flex flex-col w-full h-full overflow-y-scroll z-50 snap-y snap-mandatory",
            Hero {
                text_color,
                title: "It's time to mine.",
                subtitle: &"ORE is a fair-launch, proof-of-work, cross-border digital currency."
            }
            Block {
                title: &"Proof of work.",
                title2: &"On Solana.",
                detail: &"ORE can be mined on any laptop, phone, or home computer. You don't need any advanced hardware or a degree to get started.",
                section: Section::A,
                text_color
            }
            Block {
                title: &"Fixed supply.",
                title2: &"Predictable future.",
                detail: &"ORE has a total supply limit of 21m tokens. At a steady rate of one per minute, all ORE in existence will be mined by the year 2064.",
                section: Section::B,
                text_color
            }
            Block {
                title: &"Fair launch.",
                title2: &"Immutable code.",
                detail: &"ORE has no insider token allocation nor pre-mined supply. The smart contract is open source and audited by multiple world-class teams.",
                section: Section::C,
                text_color
                // TODO Ottersec logo
                // TODO Sec3
                // TODO Neodyme
            }
            Block {
                title: &"Borderless asset.",
                title2: &"Permissionless cash.",
                detail: &"ORE is digital money at the speed of the internet. It can be sent to anyone, anywhere in the world, in few seconds or less.",
                section: Section::D,
                text_color
                // TODO Current price (in USD, EUR, YUAN, YEN, BTC, SOL, ETH, etc.)
            }
            // Footer {}
        }
    }
}

#[component]
fn BgImg(visible: bool, bg_img: String, index: usize) -> Element {
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
    let button_color = match text_color {
        TextColor::Black => "text-black hover:bg-black hover:text-white",
        TextColor::White => "text-white hover:bg-white hover:text-black",
    };
    rsx! {
        div {
            class: "flex flex-row justify-between px-4 sm:px-8 py-4 md:py-8 w-full transition-colors duration-1000 {copy_color}",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10 my-auto",
                OreLogoIcon {
                    class: "h-6 md:h-8 my-auto"
                }
            }
            div {
                class: "flex flex-row sm:text-sm md:text-base lg:text-lg my-auto gap-8",
                Link {
                    to: "https://discord.gg/ore-supply",
                    class: "flex h-10 w-10 transition-colors rounded-full transition-colors duration-1000 {button_color}",
                    new_tab: true,
                    DiscordIcon {
                        class: "w-6 h-6 m-auto"
                    }
                }
                Link {
                    to: "https://github.com/regolith-labs/ore",
                    class: "flex h-10 w-10 transition-colors rounded-full transition-colors duration-1000 {button_color}",
                    new_tab: true,
                    GithubIcon {
                        class: "w-6 h-6 m-auto"
                    }
                }
                Link {
                    to: "https://x.com/oresupply",
                    class: "flex h-10 w-10 transition-colors rounded-full transition-colors duration-1000 {button_color}",
                    new_tab: true,
                    XIcon {
                        class: "w-5 h-5 m-auto"
                    }
                }
            }
        }
    }
}

#[component]
fn Hero(title: String, subtitle: String, text_color: TextColor) -> Element {
    let copy_color = match text_color {
        TextColor::Black => "text-black",
        TextColor::White => "text-white",
    };
    let cta_color = match text_color {
        TextColor::Black => "bg-black text-white hover:scale-105",
        TextColor::White => "bg-white text-black hover:scale-105",
    };
    rsx! {
        div {
            class: "flex flex-col min-h-svh h-full w-full snap-center snap-always",
            Navbar {
                text_color
            }
            div {
                class: "flex flex-col gap-y-8 sm:gap-y-10 md:gap-y-12 w-full md:mx-auto my-auto pb-24 px-4 md:px-8",
                div {
                    class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8 {copy_color} transition-colors duration-1000",
                    p {
                        class: "text-left sm:text-center text-6xl md:text-7xl lg:text-8xl font-bold font-hero",
                        "{title}"
                    }
                    p {
                        class: "text-left sm:text-center text-xl sm:text-2xl md:text-3xl lg:text-4xl mx-auto font-hero font-medium",
                        "{subtitle}"
                    }
                }
                Link {
                    class: "mr-auto sm:mx-auto text-center sm:text-lg md:text-xl lg:text-2xl font-semibold transition-colors transition-transform duration-200 hover:shadow {cta_color} px-6 py-3 rounded-full",
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
    rsx! {
        div {
            class: "flex min-h-svh h-full w-full snap-center",
            div {
                class: "flex flex-col h-full w-full py-16 gap-24 px-4 sm:px-8",
                div {
                    class: "flex flex-col gap-4 sm:gap-6 md:gap-8 transition-colors duration-1000 {copy_color}",
                    p {
                        class: "text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                        "{title}"
                        br {}
                        span {
                            class: "opacity-50",
                            "{title2}"
                        }
                    }
                    p {
                        class: "text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-medium font-hero",
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
                to: Route::Download {},
                "Download the app →"
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
                new_tab: true,
                "Checkout the code →"
            }
        },
        Section::D => rsx! {
            Link {
                class: "{style} {cta_color}",
                to: "https://jup.ag/swap/USDC-ORE",
                new_tab: true,
                "Buy now →"
            }
        },
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Section {
    A,
    B,
    C,
    D,
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
