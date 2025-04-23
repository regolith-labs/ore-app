use dioxus::prelude::*;

#[cfg(feature = "web")]
use crate::{
    components::*,
    hooks::{use_ore_holders, use_ore_market_cap},
    utils::{format_abbreviated_number, format_abbreviated_pubkey},
};

use crate::{gateway::ore::TopHolder, route::Route};

#[cfg(not(feature = "web"))]
pub fn Landing() -> Element {
    let navigator = use_navigator();
    navigator.replace(Route::Topup {
        address: "asdf".to_string(),
    });
    rsx! {}
}

#[cfg(feature = "web")]
pub fn Landing() -> Element {
    rsx! {
        Hero {}
        Stats {}
        Earning {}
        Mission {}
        Technology {}
        Distribution {}
        Community {}
        Faq {}
        Footer {}
    }
}

#[cfg(feature = "web")]
fn Hero() -> Element {
    rsx! {
        Col {
            // class: "relative w-full h-full max-w-screen min-h-screen 2xl:min-h-192",
            // class: "relative w-full h-full max-w-screen min-h-screen 2xl:min-h-192",
            class: "relative w-full h-full max-w-screen min-h-screen 2xl:min-h-256",
            HeroBg {}
            LandingNavbar {}
            Col {
                // class: "absolute w-full h-full mx-auto max-w-7xl top-0 bottom-0 left-0 right-0 z-50",
                class: "absolute w-full h-full mx-auto justify-end max-w-7xl top-0 bottom-0 left-0 right-0 z-50 pt-16 md:pt-32 sm:pb-8 md:pb-16",
                gap: 16,
                HeroOrb {}
                HeroTitle {}
            }
        }
    }
}

#[cfg(feature = "web")]
fn LandingNavbar() -> Element {
    rsx! {
        Row {
            class: "w-screen shrink-0 h-20 sm:h-24 px-2 sm:px-6 z-100",
            Row {
                class: "w-full my-auto justify-between",
                Logo {}
                LaunchButton {}
            }
        }
    }
}

#[cfg(feature = "web")]
fn LaunchButton() -> Element {
    rsx! {
        Link {
            // class: "flex px-8 h-12 my-auto rounded controls-primary rounded-full z-100",
            class: "flex px-8 h-12 my-auto rounded rounded-full z-100 font-sans font-semibold text-base bg-transparent hover:bg-elements-highEmphasis hover:text-black transition-all duration-300 ease-in-out text-elements-highEmphasis border-2 border-elements-highEmphasis",
            to: Route::Mine {},
            span {
                class: "my-auto",
                "Launch app"
            }
        }
    }
}

#[cfg(feature = "web")]
fn HeroTitle() -> Element {
    rsx! {
        Col {
            class: "max-w-7xl px-2 sm:px-6 mb-auto",
            gap: 4,
            Col {
                class: "gap-2 md:gap-4 font-extended font-bold text-4xl sm:text-5xl md:text-6xl lg:text-7xl text-center text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black mx-auto",
                span {
                    class: "z-30",
                    "Digital gold,"
                }
                span {
                    class: "z-30",
                    "reborn."
                }
            }
            span {
                class: "z-10 text-elements-midEmphasis leading-10 font-wide font-medium text-lg md:text-xl lg:text-2xl text-center max-w-3xl mx-auto",
                "Hard money for the age of DeFi."
            }
            Col {
                class: "md:flex-row ml-0 md:mx-auto h-min mt-4",
                gap: 4,
                Link {
                    to: Route::Mine {},
                    class: "flex h-12 w-full md:w-min px-8 rounded-full controls-primary ml-0 md:ml-auto",
                    span {
                        class: "my-auto mx-auto text-nowrap font-semibold text-base font-sans",
                        "Start mining →"
                    }
                }
                // Link {
                //     to: Route::Mine {},
                //     class: "flex h-12 w-full md:w-min px-8 rounded-full mr-0 md:mr-auto text-elements-midEmphasis hover:text-elements-highEmphasis transition-all duration-300 ease-in-out",
                //     span {
                //         class: "my-auto mx-auto text-nowrap font-semibold text-base font-sans",
                //         "Learn more"
                //     }
                // }
            }
        }
    }
}

fn Earning() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full min-h-screen md:min-h-224 mt-8 md:mt-16 font-extended font-bold text-5xl md:text-6xl lg:text-7xl text-center text-elements-highEmphasis ",
            gap: 8,
            SectionCopy {
                tip: "Earn",
                title: "One token.",
                subtitle: "Many ways to mine.",
                detail: "Find a strategy that works for you."
            }
            div {
                class: "grid auto-rows-fr grid-cols-1 md:grid-cols-2 gap-6 max-w-7xl mx-auto px-4",
                Card {
                    asset: asset!("/public/gpu.png"),
                    title: "Proof of work",
                    detail: "Convert raw available energy into ORE using a personal computer or GPU.",
                    cta: "Mine now →",
                    route: Route::Mine {}.to_string()
                }
                Card {
                    asset: asset!("/public/liquidity.png"),
                    title: "Proof of liquidity",
                    detail: "Make markets more efficient for traders by providing liquidity to the ORE ecosystem.",
                    cta: "Stake now →",
                    route: Route::Stake {}.to_string()
                }
                Card {
                    asset: asset!("/public/impressions.png"),
                    title: "Proof of post",
                    detail: "Get paid to create and share crypto content with your followers on social media.",
                    cta: "Coming soon",
                    // route: Route::Mine {}.to_string()
                }
                Card {
                    asset: asset!("/public/seeker.png"),
                    title: "Proof of mobile",
                    detail: "Use a Solana Seeker mobile phone to mine ORE anywhere and everywhere you go.",
                    cta: "Coming soon",
                    // route: "https://solanamobile.com/",
                }
            }
        }
    }
}

#[component]
fn Card(asset: Asset, title: String, detail: String, cta: String, route: Option<String>) -> Element {
    rsx! {
        if let Some(route) = route {
            Link {
                class: "bg-elements-midEmphasis/10 hover:bg-elements-midEmphasis/20 rounded-2xl cursor-pointer overflow-hidden group transition-all duration-300 ease-in-out",
                to: route,
                CardContent {
                    asset: asset,
                    title: title,
                    detail: detail,
                    cta: cta,
                }
            }
        } else {
            CardContent {
                class: "bg-elements-midEmphasis/10 hover:bg-elements-midEmphasis/20 rounded-2xl cursor-pointer overflow-hidden group transition-all duration-300 ease-in-out",
                asset: asset,
                title: title,
                detail: detail,
                cta: cta,
            }
        }
    }
}

#[component]
fn CardContent(asset: Asset, title: String, detail: String, cta: String, class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        Col {
            class: "{class}",
            span {
                class: "w-full h-72 group-hover:h-64 transition-all duration-300 ease-in-out pt-1 px-1",
                img {
                    class: "w-full h-full object-cover overflow-hidden rounded-2xl bg-transparent",
                    src: asset.to_string()
                }
            }
            Col {
                class: "px-4 md:px-6 py-4 max-w-xl mx-auto relative",
                gap: 2,
                p {
                    class: "font-wide font-semibold text-xl text-elements-highEmphasis text-left",
                    "{title}"
                }
                p {
                    class: "text-elements-midEmphasis text-lg font-sans font-normal text-left",
                    "{detail}"
                }
                p {
                    class: "text-elements-highEmphasis text-sm font-semibold font-wide text-left absolute -bottom-4 opacity-0 group-hover:opacity-100 transform translate-y-4 group-hover:translate-y-0 transition-all duration-300 ease-in-out",
                    "{cta}"
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn SectionCta(title: String, route: String) -> Element {
    rsx! {
        Col {
            // class: "sm:flex-row mx-auto md:ml-0 h-min mt-8 px-4",
            class: "sm:flex-row ml-0 h-min mt-8 px-4",
            gap: 4,
            Link {
                to: route,
                class: "flex h-12 w-full sm:w-min px-8 rounded-full controls-primary",
                span {
                    class: "my-auto mx-auto text-nowrap font-semibold",
                    "{title}"
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Mission() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen mt-16 md:mt-48 2xl:min-h-256",
            LandingGlobe {}
            Col {
                class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start md:justify-between z-10",
                Col {
                    class: "w-full h-min mx-auto max-w-7xl justify-start",
                    gap: 2,
                    SectionCopyResponsive {
                        tip: "Mission",
                        title: "Peer to peer.",
                        subtitle: "Electronic cash.",
                    }
                    span {
                        // class: "text-elements-midEmphasis text-lg text-center md:text-left px-4 -mt-4 max-w-xl mx-auto md:ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        class: "text-elements-midEmphasis text-lg text-left px-4 -mt-4 max-w-xl ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        "ORE is a public currency for the open internet — not issued or controlled by any state. It can be earned, spent, and used without banks or middlemen."
                    }
                    Data {
                        data_points: vec![
                            DataPoint {
                                title: "1 sec".to_string(),
                                detail: "Instant settlement".to_string(),
                                ore: false,
                            },
                            DataPoint {
                                title: "$0.00064".to_string(),
                                detail: "Median transaction fee".to_string(),
                                ore: false,
                            },
                            DataPoint {
                                title: "180+".to_string(),
                                detail: "Available countries".to_string(),
                                ore: false,
                            },
                        ]
                    }
                    // SectionCta {
                    //     title: "Learn more →",
                    //     route: Route::Stake {},
                    // }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DataPoint {
    title: String,
    detail: String,
    ore: bool,
}

#[component]
fn Data(data_points: Vec<DataPoint>) -> Element {
    rsx! {
        Col {
            class: "px-4 mt-8 mb-4 mx-0 text-elements-lowEmphasis text-base font-medium",
            gap: 8,
            for data_point in data_points {
                Col {
                    gap: 2,
                    Row {
                        class: "pl-4 border-l-2 border-elements-gold gap-1.5 text-elements-highEmphasis font-semibold text-2xl",
                        if data_point.ore {
                            OreIcon {
                                class: "h-5 w-5 my-auto",
                            }
                        }
                        span {
                            "{data_point.title}"
                        }
                    }
                    span {
                        class: "text-elements-lowEmphasis pl-4 border-l-2 border-transparent",
                        "{data_point.detail}"
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Technology() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen mt-80 md:mt-0 2xl:min-h-256",
            Col {
                class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start md:justify-between z-10",
                gap: 8,
                Col {
                    class: "w-full h-min mx-auto max-w-7xl justify-start",
                    gap: 2,
                    SectionCopyResponsive {
                        tip: "DeFi",
                        title: "Sound money.",
                        subtitle: "Smart currency.",
                    }
                    span {
                        // class: "text-elements-midEmphasis text-lg text-center md:text-left px-4 -mt-4 max-w-lg mx-auto md:ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        class: "text-elements-midEmphasis text-lg text-left px-4 -mt-4 max-w-lg ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        "ORE brings digital scarcity to the world of decentralized finance – fun to mine, easy to move, safe to stake, and endlessly programmable."
                    }

                    Integrations {}

                    SectionCta {
                        title: "Explore DeFi →",
                        route: Route::Stake {},
                    }
                }

                img {
                    class: "w-full h-92 md:mt-32 md:h-128 lg:h-128 rounded-2xl object-cover overflow-hidden",
                    src: asset!("/public/iphone-staking.png"),
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Integration {
    name: String,
    detail: String,
}

fn Integrations() -> Element {
    let mut selected = use_signal(|| 0);

    let data = vec![
        Integration {
            name: "Meteora".to_string(),
            detail: "Meteora powers market making and price discovery for ORE-SOL and ORE-USDC trading pairs.".to_string(),
        },
        Integration {
            name: "Orca".to_string(),
            detail: "Orca powers concentrated liquidity pools for ORE-SOL trading pairs.".to_string(),
        },
        Integration {
            name: "Kamino".to_string(), 
            detail: "Kamino provides an automated staking strategy for ORE concentrated liquidity pools.".to_string(),
        },
        Integration {
            name: "Drift".to_string(),
            detail: "Drift operates a leading perpetual futures exchange and lending platform on Solana (coming soon).".to_string(),
        },  
    ];

    let get_border_class = |idx: usize| {
        if *selected.read() == idx {
            "border-elements-highEmphasis"
        } else {
            "border-transparent"
        }
    };

    rsx! {
        Col {
            class: "px-4 mt-8",
            gap: 4,
            span {
                class: "text-elements-lowEmphasis ",
                "Integrations"
            }
            Row {
                gap: 2,
                img {
                    class: "w-12 h-12 rounded-full border-2 p-1 cursor-pointer transition-all {get_border_class(0)}",
                    src: asset!("/public/meteora_logo.jpg"),
                    onmouseenter: move |_| selected.set(0)
                }
                img {
                    class: "w-12 h-12 rounded-full border-2 p-1 cursor-pointer transition-all {get_border_class(1)}",
                    src: asset!("/public/orca-logo.png"),
                    onmouseenter: move |_| selected.set(1)
                }
                img {
                    class: "w-12 h-12 rounded-full border-2 p-1 cursor-pointer transition-all {get_border_class(2)}",
                    src: asset!("/public/kamino_logo.jpg"),
                    onmouseenter: move |_| selected.set(2)
                }
                img {
                    class: "w-12 h-12 rounded-full border-2 p-1 cursor-pointer transition-all {get_border_class(3)}",
                    src: asset!("/public/drift-logo.webp"),
                    onmouseenter: move |_| selected.set(3)
                }
            }
            Col {
                class: "max-w-md w-full pl-1",
                gap: 2,
                // span {
                //     class: "text-elements-midEmphasis font-semibold",
                //     "{data[*selected.read()].name}"
                // }
                span {
                    class: "text-elements-midEmphasis",
                    "{data[*selected.read()].detail}"
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Distribution() -> Element {
    use crate::{
        hooks::{use_ore_supply, use_ore_top_holders},
        utils::format_whole_number,
    };

    let supply = use_ore_supply();
    let holders = use_ore_holders();
    let top_holders = use_ore_top_holders();

    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen 2xl:min-h-256 mt-16 md:mt-0",
            Col {
                class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start md:justify-between z-10",
                gap: 8,
                Col {
                    class: "w-full h-min mx-auto max-w-7xl justify-start",
                    gap: 2,
                    SectionCopyResponsive {
                        tip: "Supply",
                        title: "Fair launch.",
                        subtitle: "Open source.",
                    }
                    span {
                        // class: "text-elements-midEmphasis text-lg text-center md:text-left px-4 -mt-4 max-w-lg mx-auto md:ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        class: "text-elements-midEmphasis text-lg text-left px-4 -mt-4 max-w-lg ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        "ORE has no pre-sale or insider allocations. All tokens are mined transparently via a public smart contract and owned by the community."
                    }
                    Data {
                        data_points: vec![
                            DataPoint {
                                title: if let Some(Ok(holders)) = *holders.read() {
                                    format_whole_number(holders.to_string())
                                } else {
                                    "–".to_string()
                                },
                                detail: "Token holders".to_string(),
                                ore: false,
                            },
                            DataPoint {
                                title: if let Some(Ok(supply)) = supply.cloned() {
                                    format_whole_number(supply.ui_amount_string)
                                } else {
                                    "–".to_string()
                                },
                                detail: "Circulating supply".to_string(),
                                ore: true,
                            },
                            DataPoint {
                                title: "1440".to_string(),
                                detail: "Daily emissions".to_string(),
                                ore: true,
                            },
                        ]
                    }
                    SectionCta {
                        title: "Review data →",
                        route: "https://dune.com/ore/ore",
                    }
                }
                if let Some(Ok(top_holders)) = top_holders.cloned() {
                    PieChart {
                        class: "md:pt-32",
                        data: top_holders
                    }
                } else {
                    "Loading..."
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn PieChart(class: Option<String>, data: Vec<TopHolder>) -> Element {
    // fn PieChart(class: Option<String>, data: Vec<(String, f64)>) -> Element {

    use std::str::FromStr;

    use steel::Pubkey;
    let class = class.unwrap_or_default();
    let total: f64 = data.iter().map(|top_holder| top_holder.balance).sum();
    let mut current_angle = 0.0;
    let mut selected = use_signal(|| 0_usize);

    let mut path_data = Vec::new();
    for (idx, top_holder) in data.iter().enumerate() {
        let percentage = top_holder.balance / total;
        let angle = percentage * 360.0;
        let end_angle = current_angle + angle;

        // Outer arc
        let outer_start_x = (current_angle * std::f64::consts::PI / 180.0).cos();
        let outer_start_y = (current_angle * std::f64::consts::PI / 180.0).sin();
        let outer_end_x = (end_angle * std::f64::consts::PI / 180.0).cos();
        let outer_end_y = (end_angle * std::f64::consts::PI / 180.0).sin();

        // Inner arc (scaled down to 0.6 of outer radius)
        let inner_start_x = outer_start_x * 0.6;
        let inner_start_y = outer_start_y * 0.6;
        let inner_end_x = outer_end_x * 0.6;
        let inner_end_y = outer_end_y * 0.6;

        let large_arc = if angle > 180.0 { 1 } else { 0 };

        path_data.push((
            format!(
                "M {} {} A 1 1 0 {} 1 {} {} L {} {} A 0.6 0.6 0 {} 0 {} {} Z",
                outer_start_x,
                outer_start_y,
                large_arc,
                outer_end_x,
                outer_end_y,
                inner_end_x,
                inner_end_y,
                large_arc,
                inner_start_x,
                inner_start_y
            ),
            top_holder.address.clone(),
            percentage,
            idx,
            outer_start_x,
            outer_start_y,
            outer_end_x,
            outer_end_y,
        ));

        current_angle = end_angle;
    }

    let get_scale_class = |idx: usize| {
        if *selected.read() == idx {
            "scale-105"
        } else {
            ""
        }
    };

    rsx! {
        Col {
            class: "{class}",
            gap: 4,
            svg {
                class: "w-full h-full md:h-96 md:w-96 lg:h-128 lg:w-128",
                view_box: "-1.2 -1.2 2.4 2.4",
                style: "transform: rotate(-90deg)",
                for (path_d, _label, _percentage, idx, _outer_start_x, _outer_start_y, _outer_end_x, _outer_end_y) in path_data.clone() {
                    path {
                        d: "{path_d}",
                        fill: if *selected.read() == idx { "#ECC771" } else { get_color(idx) },
                        // class: "transition-all duration-300 cursor-pointer hover:scale-105",
                        class: "transition-all duration-300 cursor-pointer {get_scale_class(idx)}",
                        onmouseover: move |_| {
                            selected.set(idx);
                        }
                    }
                }
            }

            if let Some((_, label, percentage, _idx, _, _, _, _)) = path_data.get(*selected.read()) {
                Col {
                    class: "mx-auto text-center",
                    gap: 2,
                    span {
                        class: "text-lg font-semibold text-elements-highEmphasis",
                        if let Ok(address) = Pubkey::from_str(&label) {
                            "{format_abbreviated_pubkey(address)}"
                        } else {
                            "{label}"
                        }
                    }
                    span {
                        class: "text-lg text-elements-midEmphasis",
                        "{(percentage * 100.0):.5}%"
                    }
                }
            }
        }
    }
}

fn get_color(index: usize) -> &'static str {
    match index % 6 {
        0 => "rgba(142, 142, 147, 0.8)",
        1 => "rgba(99, 99, 102, 0.8)",
        2 => "rgba(72, 72, 74, 0.8)",
        3 => "rgba(58, 58, 60, 0.8)",
        4 => "rgba(44, 44, 46, 0.8)",
        5 => "rgba(28, 28, 30, 0.8)",
        _ => "rgba(0, 0, 0, 0.8)",
    }
}

#[cfg(feature = "web")]
fn Community() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-7xl pt-32 md:pt-0",
            SectionCopy {
                tip: "Community",
                title: "Join the movement.",
                detail: "Discover why thousands of people around the world love ORE."
            }
            Testimonials {}
        }
    }
}

#[cfg(feature = "web")]
#[derive(Clone, PartialEq)]
struct TestimonialData {
    image: String,
    name: String,
    quote: String,
    link: String,
}

#[cfg(feature = "web")]
fn Testimonials() -> Element {
    let data = vec![
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1896990528748593152/jU2rStOc_400x400.jpg".into(),
            name: "Anatoly Yakovenko".into(),
            quote: ".OREsupply is cool".into(),
            link: "https://x.com/aeyakovenko/status/1891891612235727093".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1857467519042232321/GLvZxG-T_400x400.jpg".into(),
            name: "network state enjoyooor".into(),
            quote: "ORE becoming more gamified with every new update . This is what mining evolution looks like.".into(),
            link: "https://x.com/lowercaseben/status/1878117108287943112".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1830469870200193024/7xI_DeCq_400x400.jpg".into(),
            name: "Brewtoshi".into(),
            quote: "Ore Boosts are its killer feature and are super underrated atm".into(),
            link: "https://x.com/Brewtoshi/status/1875560756332392708".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1876410450767925248/J-l8lpL6_400x400.jpg".into(),
            name: "Ore Historian".into(),
            quote: "Its not crazy and it will work. $ORE will be running the defi ecosystem".into(),
            link: "https://x.com/oreHistorian/status/1877146737673981959".into()        
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1651271535800336406/vR1FxvDs_400x400.jpg".into(),
            name: "Matty Tay".into(),
            quote: "BTC walked so ORE could run.".into(),
            link: "https://x.com/mattytay/status/1870887900663169059".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1815598662006792192/ShUElYCu_400x400.jpg".into(),
            name: "kel".into(),
            quote: "using proof of work for distribution atop a performant proof of stake chain has the potential to be the next such faded mechanism".into(),
            link: "https://x.com/kelxyz_/status/1819423305096425812".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1828429925428158464/DgmDex35_400x400.jpg".into(),
            name: "Vidiu".into(),
            quote: "The answer is simple @OREsupply".into(),
            link: "https://x.com/0xVidiu/status/1892670871984062474".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1848549429717975040/JpZUMEAW_400x400.jpg".into(),
            name: "Elias".into(),
            quote: "ore is actually here to help us. it can only make the network stronger".into(),
            link: "https://x.com/Eliascm17/status/1776341784118890765".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1798717628099469312/TiVu101s_400x400.jpg".into(),
            name: "Farhaj Mayan".into(),
            quote: "Upgraded my ORE. LFG 🫡 @OREsupply".into(),
            link: "https://x.com/farhajmayan/status/1820073386107720121".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1873772860566638592/cTfnGR67_400x400.jpg".into(),
            name: "SOL Big Brain".into(),
            quote: "Been in heavy accumulation mode of $ORE (@OREsupply) lately.".into(),
            link: "https://x.com/SOLBigBrain/status/1870124964088533248".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1510345561731330063/mRH8nY7D_400x400.jpg".into(),
            name: "Madhatt3r".into(),
            quote: "Memecoins will come and go but ORE is forever. It is hard money in a sea of inflationary credit. Believe in something.".into(),
            link: "https://x.com/".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1853830577872347136/7fDP-JKR_400x400.jpg".into(),
            name: "Solana Legend ".into(),
            quote: "Born too late to own a house. Born too early to be a TikTok star. Born at the perfect time to mine ORE".into(),
            link: "https://x.com/SolanaLegend/status/1820629234232000721".into()
        },
        TestimonialData {
            image: "https://pbs.twimg.com/profile_images/1536022035435991046/Ih9CZm-r_400x400.jpg".into(),
            name: "David Chapman".into(),
            quote: "ORE is what Satoshi envisioned.".into(),
            link: "https://x.com/DChapmanCrypto/status/1820710738308280432".into()
        },
    ];

    rsx! {
        Col {
            class: "w-full h-min mx-auto max-w-7xl justify-start mt-8",
            TestimonialWall {
                class: "hidden sm:flex",
                testimonial_data: data.clone()
            }
            TestimonialList {
                class: "sm:hidden",
                testimonial_data: data.clone()
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn TestimonialList(class: Option<String>, testimonial_data: Vec<TestimonialData>) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        Row {
            class: "w-full gap-4 overflow-x-auto px-4 {class}",
            for data in testimonial_data {
                Testimonial {
                    class: "my-auto min-w-64",
                    data: data.clone()
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn TestimonialWall(class: Option<String>, testimonial_data: Vec<TestimonialData>) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        Row {
            class: "w-full gap-4 px-4 {class}",
            Col {
                class: "my-auto gap-4",
                for data in testimonial_data[0..4] {
                    Testimonial {
                        data: data.clone()
                    }
                }
            }
            Col {
                class: "gap-4",
                for data in testimonial_data[4..9] {
                    Testimonial {
                        data: data.clone()
                    }
                }
            }
            Col {
                class: "my-auto gap-4",
                for data in testimonial_data[9..13] {
                    Testimonial {
                        data: data.clone()
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn Testimonial(class: Option<String>, data: TestimonialData) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        a {
            // class: "flex flex-col bg-elements-midEmphasis/10 rounded-md p-5 border-2 border-transparent hover:border-elements-highEmphasis transition-all duration-300 ease-in-out grow {class}",
            class: "flex flex-col bg-elements-midEmphasis/10 hover:bg-elements-midEmphasis/20 rounded-md p-5 transition-all duration-300 ease-in-out grow {class}",
            href: "{data.link}",
            target: "_blank",
            Row {
                class: "gap-3",
                img {
                    class: "w-10 h-10 rounded-full",
                    src: "{data.image}" // Placeholder avatar
                }
                Col {
                    class: "gap-1",
                    span {
                        class: "font-semibold text-elements-highEmphasis",
                        "{data.name}"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "{data.quote}"
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Stats() -> Element {
    // TODO
    let holders = use_ore_holders();
    let market_cap = use_ore_market_cap();
    rsx! {
        Col {
            class: "w-full h-min text-elements-highEmphasis px-4",
            Col {
                class: "md:flex-row md:gap-32 relative w-full py-16 md:py-32 px-4 mx-auto max-w-7xl border-t-2 border-b-2 border-elements-midEmphasis selection:bg-elements-highEmphasis selection:text-black",
                gap: 16,
                Col {
                    class: "md:ml-auto",
                    gap: 2,
                    span {
                        class: "text-4xl md:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        if let Some(Ok(holders)) = holders.cloned() {
                            "{format_abbreviated_number(holders as f64)}"
                        } else {
                            "–"
                        }
                    }
                    span {
                        class: "text-lg md:text-xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Holders"
                    }
                }

                Col {
                    class: "text-left",
                    gap: 2,
                    span {
                        class: "text-4xl md:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        "5,000,000"
                    }
                    span {
                        class: "text-lg md:text-xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Max supply"
                    }
                }

                Col {
                    class: "md:mr-auto",
                    gap: 2,
                    span {
                        class: "text-4xl md:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        if let Some(Ok(market_cap)) = market_cap.cloned() {
                            "${format_abbreviated_number(market_cap)}"
                        } else {
                            "–"
                        }
                    }
                    span {
                        class: "text-lg md:text-xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Market cap"
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Faq() -> Element {
    rsx! {
        Col {
            class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start mt-32 px-0 sm:px-4",
            SectionCopy {
                class: "text-left md:min-w-sm lg:min-w-md",
                align: Align::Left,
                tip: "Support",
                title: "FAQ",
                detail: "Get answers to common questions."
            }
            Col {
                class: "w-full h-min justify-start md:mt-16",
                FaqItem {
                    question: "What is ORE?",
                    answer: "ORE is a new \"digital gold\" primitive for decentralized finance. It is a scarce crypto commodity, mineable via proof-of-work on the Solana blockchain."
                }
                FaqItem {
                    question: "Why should I care?",
                    answer: "Only a small handful of people in the world can truly afford to mine and use digital gold on the Bitcoin blockchain. ORE is digital gold for everyone else –  a hard money asset that is fun to mine, easy to use, and natively interoperable with decentralized finance."
                }
                FaqItem {
                    question: "How does mining work?",
                    answer: "There are a variety of ways to mine ORE. Users can deploy a home computer or GPU to convert available hashpower into tokens. Users can even mine on special mobile devices such as the Solana Seeker, earn tokens by providing liquidity in public markets, or even get paid to post content on social media."
                }
                FaqItem {
                    question: "Is it secure?",
                    answer: "ORE has been thoroughly audited by two independent auditing firms. The smart contracts are open source and have been battled tested in production. The development team is committed to permanently freezing the protocol in the near future to guarantee longterm security."
                }
            }
        }
    }
}

#[cfg(feature = "web")]
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
            class: "flex flex-col w-full py-8 px-4 sm:px-8 cursor-pointer transition-all duration-300 ease-in-out rounded-md hover:bg-elements-midEmphasis/10",
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "justify-between font-wide text-left font-bold text-xl w-full text-elements-highEmphasis",
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

#[cfg(feature = "web")]
#[derive(Clone, PartialEq)]
enum Align {
    Left,
    Center,
}

#[cfg(feature = "web")]
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
            class: "py-8 sm:gap-2 font-wide font-bold text-3xl sm:text-4xl md:text-5xl lg:text-6xl text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black px-4 {class} {text_align}",
            // gap: 2,
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
                    class: "md:mb-auto mt-4 z-10 text-elements-midEmphasis font-wide font-medium text-base sm:text-lg md:text-xl lg:text-2xl {text_margin}",
                    "{detail}"
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn SectionCopyResponsive(tip: Option<String>, title: String, subtitle: Option<String>) -> Element {
    rsx! {
        // SectionCopy {
        //     class: "hidden md:flex w-full text-nowrap",
        //     align: Align::Left,
        //     tip: tip.clone(),
        //     title: title.clone(),
        //     subtitle: subtitle.clone(),
        // }
        // SectionCopy {
        //     class: "md:hidden",
        //     tip: tip.clone(),
        //     title: title.clone(),
        //     subtitle: subtitle.clone(),
        // }
        SectionCopy {
            class: "flex w-full text-nowrap",
            align: Align::Left,
            tip: tip.clone(),
            title: title.clone(),
            subtitle: subtitle.clone(),
        }
        // SectionCopy {
        //     class: "md:hidden",
        //     tip: tip.clone(),
        //     title: title.clone(),
        //     subtitle: subtitle.clone(),
        // }
    }
}

#[cfg(feature = "web")]
fn Footer() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full text-elements-highEmphasis pt-32 px-4",
            Row {
                class: "w-full h-min mx-auto max-w-7xl justify-end pb-4",
                SocialLinks {}
            }
            Col {
                class: "w-full h-min mx-auto max-w-7xl border-t-2 border-elements-midEmphasis pt-4",
                gap: 16,
                Row {
                    class: "w-full h-min mx-auto max-w-7xl justify-end px-2",
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis text-sm font-medium",
                        "© Regolith Labs 2025."
                    }
                    // span {
                    //     class: "text-elements-lowEmphasis text-sm font-medium",
                    //     "Made in America."
                    // }
                }
                OreWordmarkIcon {
                    class: "w-full"
                }
            }
        }
    }
}
