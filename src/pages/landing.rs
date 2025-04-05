use dioxus::prelude::*;

#[cfg(feature = "web")]
use crate::{
    components::*,
    hooks::{use_ore_holders, use_ore_market_cap},
    utils::format_abbreviated_number,
};

use crate::route::Route;

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
        Mining {}
        Liquidity {}
        Stats {}
        Community {}
        Faq {}
        Footer {}
    }
}

#[cfg(feature = "web")]
fn Hero() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full max-w-screen min-h-screen 2xl:min-h-192",
            LandingNavbar {}
            Col {
                class: "absolute w-full h-full mx-auto max-w-7xl top-0 bottom-0 left-0 right-0 z-50",
                HeroTitle {}
                HeroOrb {}
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
            class: "flex px-8 h-12 my-auto rounded controls-primary rounded-full z-100",
            to: Route::Mine {},
            span {
                class: "my-auto",
                "Launch app →"
            }
        }
    }
}

#[cfg(feature = "web")]
fn HeroTitle() -> Element {
    rsx! {
        Col {
            class: "absolute left-0 right-0 bottom-16 sm:left-8 sm:right-8 md:bottom-0 md:top-0 font-extended font-bold text-7xl md:text-8xl lg:text-9xl text-center md:text-left text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black",
            gap: 2,
            span {
                class: "md:mt-auto z-30",
                "Liquid"
            }
            span {
                class: "z-20",
                "Digital"
            }
            span {
                class: "mb-4 md:mb-8 z-10",
                "Gold"
            }
            span {
                class: "md:mb-auto z-10 text-elements-midEmphasis leading-12 font-wide font-medium text-2xl md:text-3xl lg:text-4xl text-center md:text-left max-w-2xl",
                "Convert energy into cryptocurrency."
            }
        }
    }
}

#[cfg(feature = "web")]
fn Mining() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full min-h-screen md:min-h-224 mt-16",
            Col {
                class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start md:justify-between",
                Col {
                    gap: 8,
                    SectionCopyResponsive {
                        tip: "Fair launch",
                        title: "Proof of work.",
                        subtitle: "On Solana.",
                    }
                    MiningGuide {}
                    SectionCtas {
                        primary_title: "Start mining →",
                        primary_route: Route::Mine {},
                        secondary_title: "Learn more",
                        secondary_route: Route::Mine {}
                    }
                }
                MiningIllustration {}
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn SectionCtas(
    primary_title: String,
    primary_route: Route,
    secondary_title: String,
    secondary_route: Route,
) -> Element {
    rsx! {
        Col {
            class: "sm:flex-row mx-auto md:ml-0 h-min mt-8 px-4",
            gap: 4,
            Link {
                to: primary_route,
                class: "flex h-12 w-full sm:w-min px-8 rounded-full controls-primary",
                span {
                    class: "my-auto mx-auto text-nowrap font-semibold",
                    "{primary_title}"
                }
            }
            // Link {
            //     to: secondary_route,
            //     class: "flex h-12 w-full sm:w-min px-8 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis transition-colors hover:bg-controls-tertiaryHover duration-300 ease-in-out",
            //     span {
            //         class: "my-auto mx-auto text-nowrap font-semibold",
            //         "{secondary_title}"
            //     }
            // }
        }
    }
}

#[cfg(feature = "web")]
fn MiningIllustration() -> Element {
    rsx! {
        div {
            class: "relative h-160 w-screen md:w-auto overflow-hidden shrink-0 pointer-events-none",
            div {
                class: "absolute inset-0 z-0",
                HashAnimation {}
            }
            img {
                class: "relative w-full h-full pb-8 pt-8 object-contain z-10",
                src: asset!("/public/rock-phone.png")
            }
        }
    }
}

#[cfg(feature = "web")]
fn HashAnimation() -> Element {
    let mut hash_text = use_signal(|| "".to_string());
    let length = 512;
    let _batch_size = 64;
    let _update_interval = 500;
    let chars = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    use_effect(move || {
        // Initialize with random characters
        let mut current_hash = String::with_capacity(length);
        for _ in 0..length {
            let idx = fastrand::usize(..chars.len());
            current_hash.push(chars.chars().nth(idx).unwrap());
        }
        hash_text.set(current_hash.clone());

        // spawn(async move {
        // Change 10 random positions
        // loop {
        //     let mut new_hash = current_hash.clone();
        //     for _ in 0..batch_size {
        //         let pos = fastrand::usize(..length);
        //         let idx = fastrand::usize(..chars.len());
        //         new_hash.replace_range(pos..pos+1, &chars.chars().nth(idx).unwrap().to_string());
        //     }
        //     current_hash = new_hash;
        //     hash_text.set(current_hash.clone());
        //     async_std::task::sleep(std::time::Duration::from_millis(update_interval)).await;
        // }
        // });
    });

    rsx! {
        Col {
            class: "absolute opacity-20 font-mono font-semibold text-5xl text-elements-lowEmphasis whitespace-normal break-words top-0 left-0 md:right-0 -right-16 bottom-0 z-10",
            span {
                class: "bottom-0",
                "{hash_text}"
            }
        }
        Col {
            class: "absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-b md:bg-gradient-to-r from-black to-transparent z-20",
        }
    }
}

#[cfg(feature = "web")]
fn MiningGuide() -> Element {
    rsx! {
        Col {
            class: "md:w-lg h-min mx-auto md:mr-auto md:ml-0 px-4",
            gap: 8,
            GuideStep {
                step: "1",
                title: "Connect wallet",
                detail: "Authenticate with any Solana wallet."
            }
            GuideStep {
                step: "2",
                title: "Join a pool",
                detail: "Lower costs and avoid transaction fees."
            }
            GuideStep {
                step: "3",
                title: "Mine crypto",
                detail: "Earn rewards with just a computer."
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn GuideStep(step: String, title: String, detail: String) -> Element {
    rsx! {
        Row {
            class: "w-full h-min",
            gap: 4,
            div {
                class: "flex text-sm h-8 w-8 font-semibold border border-elements-lowEmphasis shrink-0 rounded",
                span {
                    class: "mx-auto my-auto w-min h-min text-center",
                    "{step}"
                }
            }
            Col {
                class: "w-full h-min",
                div {
                    class: "flex text-xl font-semibold text-elements-highEmphasis h-8",
                    span {
                        class: "my-auto",
                        "{title}"
                    }
                }
                span {
                    class: "text-lg text-elements-midEmphasis",
                    "{detail}"
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Liquidity() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen md:min-h-192 px-4 mt-16",
            img {
                class: "absolute left-0 right-0 bottom-0 mx-auto max-w-7xl w-full object-contain z-0",
                src: asset!("/public/ribbon.png")
            }
            Col {
                class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start md:justify-between z-10",
                Col {
                    class: "w-full h-min mx-auto max-w-7xl justify-start",
                    gap: 8,
                    SectionCopyResponsive {
                        tip: "Defi",
                        title: "Deep liquidity.",
                        subtitle: "Native yield.",
                    }
                    span {
                        class: "text-elements-midEmphasis text-lg text-center md:text-left px-4 -mt-4 max-w-xl mx-auto md:ml-0 selection:bg-elements-highEmphasis selection:text-black",
                        "ORE generates longterm yield for stakers. Farm competitive yield rates by providing liquidity for ORE markets."
                    }
                    SectionCtas {
                        primary_title: "Explore staking →",
                        primary_route: Route::Stake {},
                        secondary_title: "Learn more",
                        secondary_route: Route::Stake {}
                    }
                    // LandingWave {}
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn Community() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-7xl pt-16",
            SectionCopy {
                tip: "Social",
                title: "Join the community.",
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
            class: "flex flex-col bg-elements-midEmphasis/10 rounded-md p-5 border-2 border-transparent hover:border-elements-highEmphasis transition-all duration-300 ease-in-out grow {class}",
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
                class: "md:flex-row md:gap-24 relative w-full py-16 md:py-32 px-4 mx-auto max-w-7xl border-t border-b border-elements-midEmphasis selection:bg-elements-highEmphasis selection:text-black",
                gap: 16,
                Col {
                    class: "md:ml-auto",
                    span {
                        class: "text-4xl lg:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        if let Some(Ok(holders)) = holders.cloned() {
                            "{format_abbreviated_number(holders as f64)}"
                        } else {
                            "–"
                        }
                    }
                    span {
                        class: "text-lg md:text-xl lg:text-2xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Holders"
                    }
                }

                Col {
                    class: "text-left",
                    span {
                        class: "text-4xl lg:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        "5,000,000"
                    }
                    span {
                        class: "text-lg md:text-xl lg:text-2xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Max Supply"
                    }
                }

                Col {
                    class: "md:mr-auto",
                    span {
                        class: "text-4xl lg:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        if let Some(Ok(market_cap)) = market_cap.cloned() {
                            "${format_abbreviated_number(market_cap)}"
                        } else {
                            "–"
                        }
                    }
                    span {
                        class: "text-lg md:text-xl lg:text-2xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Market Cap"
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
                detail: "Commonly asked questions."
            }
            Col {
                class: "w-full h-min justify-start md:mt-16",
                FaqItem {
                    question: "What is ORE?",
                    answer: "ORE is digital commodity, mineable via proof-of-work on Solana."
                }
                FaqItem {
                    question: "Why should I care?",
                    answer: "ORE allows anyone to convert electricity and spare compute resources into liquid financial capital. This allows data centers to monetize underutilized compute capacity, and individuals to permissionlessly onramp to Solana without relying on financial institutions."
                }
                FaqItem {
                    question: "How does mining work?",
                    answer: "ORE can be mined by anyone with a laptop or home computer. Simply navigate to the mining page of the app, connect your Solana wallet, and click the \"Start\" button. You will automatically be enrolled in a mining pool and do not need to pay any transaction fees while you mine."
                }
                FaqItem {
                    question: "How does staking work?",
                    answer: "ORE automatically distributes a portion of all newly mined supply to liquidity providers as yield. These incentives help bootstrap liquidity and maintain active markets with a network of assets in the Solana ecosystem. By providing liquidity for ORE pairs, stakers can earn fees from traders as well as receive additional rewards in the form of ORE yield."
                }
                FaqItem {
                    question: "Is it secure?",
                    answer: "ORE has been thoroughly audited by two independent auditing firms. The code is open source and has been battled tested in production. The development team is committed to permenantly freezing the protocol in the near future to guarantee longterm security."
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
                class: "justify-between font-wide text-left font-bold text-2xl w-full text-elements-highEmphasis",
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

#[cfg(feature = "web")]
#[component]
fn SectionCopyResponsive(tip: Option<String>, title: String, subtitle: Option<String>) -> Element {
    rsx! {
        SectionCopy {
            class: "hidden md:flex w-full text-nowrap",
            align: Align::Left,
            tip: tip.clone(),
            title: title.clone(),
            subtitle: subtitle.clone(),
        }
        SectionCopy {
            class: "md:hidden",
            tip: tip.clone(),
            title: title.clone(),
            subtitle: subtitle.clone(),
        }
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
                class: "w-full h-min mx-auto max-w-7xl border-t border-elements-midEmphasis pt-4",
                gap: 16,
                Row {
                    class: "w-full h-min mx-auto max-w-7xl justify-end px-2",
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis text-sm font-medium",
                        "© Regolith Labs 2025."
                    }
                    span {
                        class: "text-elements-lowEmphasis text-sm font-medium",
                        "Made in America."
                    }
                }
                OreWordmarkIcon {
                    class: "w-full"
                }
            }
        }
    }
}
