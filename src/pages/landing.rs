use dioxus::prelude::*;

use crate::{components::*, hooks::{use_ore_holders, use_ore_market_cap}, route::Route, utils::format_abbreviated_number};

pub fn Landing() -> Element {
    rsx! {
        Hero {}
        // Marqee {}
        Mining {}
        Liquidity {}
        Stats {}
        Community {}
        Faq {}
        Footer {}
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full max-w-screen min-h-192",
            LandingNavbar {}
            Col {
                class: "absolute w-full h-full mx-auto max-w-7xl top-0 bottom-0 left-0 right-0 z-50",
                HeroTitle {}
                HeroOrb {}
            }
        }
    }
}

fn LandingNavbar() -> Element {
    rsx! {
        Row {
            class: "w-screen shrink-0 h-16 sm:h-24 px-2 sm:px-6 z-100",
            Row {
                class: "w-full my-auto justify-between",
                Logo {}
                LaunchButton {}
            }
        }
    }
}

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

fn HeroTitle() -> Element {
    rsx! {
        Col {
            class: "absolute left-0 right-0 bottom-32 sm:left-8 sm:right-8 md:bottom-0 md:top-0 font-extended font-bold text-7xl md:text-8xl lg:text-9xl text-center md:text-left text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black",
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
                class: "md:mb-auto z-10",
                "Gold" 
            }
        }
    }
}
fn Mining() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen md:min-h-224 px-4 mt-16",
            HashAnimation {}
            Col {
                // class: "relative w-full h-min mx-auto max-w-7xl justify-start",
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    class: "bg-gradient-to-r from-transparent via-black to-transparent from-10% via-50% to-90% z-10",
                    tip: "Fair launch",
                    title: "Proof of work.",
                    subtitle: "On Solana.", 
                    detail: "Start mining crypto in just one click."
                }
                LandingMiner {}
            }
        }
    }
}

fn HashAnimation() -> Element {
    let mut hash_text = use_signal(|| "".to_string());
    let chars = "0123456789abcdef";
    
    use_effect(move || {
        spawn(async move {
            loop {
                // Generate 64 character random hex string
                let mut new_hash = String::with_capacity(64);
                for _ in 0..64 {
                    let idx = fastrand::usize(..chars.len());
                    new_hash.push(chars.chars().nth(idx).unwrap());
                }
                hash_text.set(new_hash);
                async_std::task::sleep(std::time::Duration::from_millis(1000)).await;
            }
        });
    });

    rsx! {
        Col {
            class: "absolute bottom-0 left-1/2 -translate-x-1/2 w-min w-full h-full overflow-hidden opacity-10 pointer-events-none z-0",
            Col {
                class: "w-full gap-4 mt-auto font-mono font-semibold text-5xl text-elements-lowEmphasis whitespace-pre-wrap",
                for _ in 0..32 {
                    span {
                        "{hash_text}"
                    }
                }
            }
        }
    }
}

fn Liquidity() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen md:min-h-192 px-4 mt-16",
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    tip: "Defi",
                    title: "Deep liquidity.",
                    subtitle: "Native yield.",
                    detail: "Stake your crypto and earn yield."
                }
                LandingWave {}
            }
        }
    }
}

fn Community() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-7xl min-h-192 pt-16 px-4",
            SectionCopy {
                tip: "Social",
                title: "Join the community.",
                detail: "Discover why people around the world love ORE."
            }
            Testimonials {}
        }
    }
}
fn Testimonials() -> Element {
    rsx! {
        Col {
            class: "w-full h-min mx-auto max-w-7xl justify-start mt-8",
            Row {
                class: "w-full gap-4",
                Col {
                    class: "flex-1 mt-16",
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1651271535800336406/vR1FxvDs_400x400.jpg",
                        name: "Matty Tay",
                        quote: "BTC walked so ORE could run.",
                        link: "https://x.com/mattytay/status/1870887900663169059"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1857467519042232321/GLvZxG-T_400x400.jpg",
                        name: "network state enjoyooor",
                        quote: "ORE becoming more gamified with every new update . This is what mining evolution looks like.",
                        link: "https://x.com/lowercaseben/status/1878117108287943112"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1830469870200193024/7xI_DeCq_400x400.jpg",
                        name: "Brewtoshi",
                        quote: "Ore Boosts are its killer feature and are super underrated atm",
                        link: "https://x.com/Brewtoshi/status/1875560756332392708"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1876410450767925248/J-l8lpL6_400x400.jpg",
                        name: "Ore Historian",
                        quote: "Its not crazy and it will work. $ORE will be running the defi ecosystem",
                        link: "https://x.com/oreHistorian/status/1877146737673981959"
                    }
                }
                Col {
                    class: "flex-1",
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1892613729700691968/cG64Yc06_400x400.jpg",
                        name: "Anatoly Yakovenko",
                        quote: ".OREsupply is cool",
                        link: "https://x.com/aeyakovenko/status/1891891612235727093"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1815598662006792192/ShUElYCu_400x400.jpg",
                        name: "kel",
                        quote: "using proof of work for distribution atop a performant proof of stake chain has the potential to be the next such faded mechanism",
                        link: "https://x.com/kelxyz_/status/1819423305096425812"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1828429925428158464/DgmDex35_400x400.jpg",
                        name: "Vidiu",
                        quote: "The answer is simple @OREsupply",
                        link: "https://x.com/0xVidiu/status/1892670871984062474"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1848549429717975040/JpZUMEAW_400x400.jpg",
                        name: "Elias",
                        quote: "ore is actually here to help us. it can only make the network stronger",
                        link: "https://x.com/Eliascm17/status/1776341784118890765"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1798717628099469312/TiVu101s_400x400.jpg",
                        name: "Farhaj Mayan",
                        quote: "Upgraded my ORE. LFG 🫡 @OREsupply",
                        link: "https://x.com/farhajmayan/status/1820073386107720121"
                    }
                }
                Col {
                    class: "flex-1 mt-16",
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1873772860566638592/cTfnGR67_400x400.jpg",
                        name: "SOL Big Brain",
                        quote: "Been in heavy accumulation mode of $ORE (@OREsupply) lately.",
                        link: "https://x.com/SOLBigBrain/status/1870124964088533248"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1510345561731330063/mRH8nY7D_400x400.jpg",
                        name: "Madhatt3r",
                        quote: "Memecoins will come and go but ORE is forever. It is hard money in a sea of inflationary credit. Believe in something.",
                        link: "https://x.com/"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1853830577872347136/7fDP-JKR_400x400.jpg",
                        name: "Solana Legend ",
                        quote: "Born too late to own a house. Born too early to be a TikTok star. Born at the perfect time to mine ORE",
                        link: "https://x.com/SolanaLegend/status/1820629234232000721"
                    }
                    Testimonial {
                        image: "https://pbs.twimg.com/profile_images/1536022035435991046/Ih9CZm-r_400x400.jpg",
                        name: "David Chapman",
                        quote: "ORE is what Satoshi envisioned.",
                        link: "https://x.com/DChapmanCrypto/status/1820710738308280432"
                    }
                }
            }
        }
    }
}

#[component]
fn Testimonial(image: String, name: String, quote: String, link: String) -> Element {
    rsx! {
        a {
            class: "flex flex-col bg-elements-midEmphasis/10 rounded-lg p-5 mb-4 border-2 border-transparent hover:border-elements-highEmphasis transition-all duration-300 ease-in-out",
            href: "{link}",
            target: "_blank",
            Row {
                class: "gap-3",
                img {
                    class: "w-10 h-10 rounded-full",
                    src: "{image}" // Placeholder avatar
                }
                Col {
                    class: "gap-1",
                    span {
                        class: "font-semibold text-elements-highEmphasis",
                        "{name}"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "{quote}"
                    }
                }
            }
        }
    }
}

fn Stats() -> Element {
    // TODO 
    let holders = use_ore_holders();
    let market_cap = use_ore_market_cap();
    rsx! {
        Col {
            class: "w-full h-min text-elements-highEmphasis px-4",
            Col {
                class: "md:flex-row md:gap-24 relative w-full py-16 md:py-32 px-4 mx-auto max-w-7xl border-t border-b border-elements-midEmphasis",
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

fn Faq() -> Element {
    rsx! {
        Col {
            class: "md:flex-row w-full h-min mx-auto max-w-7xl justify-start mt-16 px-4",
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
                    answer: "ORE is a new \"digital gold\" primitive for decentralized finance. It is a crypto commodity mineable via proof-of-work on the Solana blockchain."
                }
                FaqItem {
                    question: "Why should I care?",
                    answer: "Bitcoin defi is broken. L2s, bridges, and custodians are fragmented Bitcoin's liquidity across defi. ORE is an experiment to bring the best ideas of Bitcoin to the Solana defi ecosystem."
                }
                FaqItem {
                    question: "How does mining work?",
                    answer: "TODO."
                }
                FaqItem {
                    question: "How does staking work?",
                    answer: "TODO."
                }
                FaqItem {
                    question: "Where does the yield come from?",
                    answer: "TODO."
                }
            }
        }
    }
}
#[component]
fn FaqItem(question: String, answer: String) -> Element {
    let mut is_open = use_signal(|| false);
    let rotation = if is_open.cloned() { "rotate-45" } else { "rotate-0" };
    rsx! {
        button {
            class: "flex flex-col py-8 px-8 cursor-pointer transition-all duration-300 ease-in-out rounded hover:bg-elements-midEmphasis/10",
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "justify-between font-wide text-left font-bold text-2xl text-elements-highEmphasis",
                gap: 8,
                "{question}"
                PlusIcon {
                    class: "w-4 h-4 my-auto transition-transform duration-300 ease-in-out text-elements-lowEmphasis {rotation}"
                }
            }
            div {
                class: "overflow-hidden transition-all duration-300 ease-in-out",
                style: if is_open.cloned() { "max-height: 200px; opacity: 1;" } else { "max-height: 0px; opacity: 0;" },
                p {
                    class: "text-elements-midEmphasis mt-4 text-lg text-left",
                    "{answer}"
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

#[component]
fn SectionCopy(class: Option<String>, align: Option<Align>, tip: Option<String>, title: String, subtitle: Option<String>, detail: Option<String>) -> Element {
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
                    class: "z-30 border-2 border-elements-gold text-elements-gold rounded-full w-min px-4 py-2 text-xs font-semibold mb-4 text-nowrap {text_margin}",
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

fn Footer() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full text-elements-highEmphasis pt-32 px-4",
            Row {
                class: "w-full h-min mx-auto max-w-7xl justify-end pb-4",
                SocialLinks {}
            }
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-between border-t border-elements-midEmphasis pt-4",
                // class: "w-full h-min mx-auto max-w-7xl justify-between pt-4",
                gap: 16,
                span {
                    class: "text-elements-lowEmphasis text-sm font-medium ml-auto mr-4",
                    "© Regolith Labs 2025"
                }
                OreWordmarkIcon {
                    class: "w-full"
                }
            }
        }
    }
}