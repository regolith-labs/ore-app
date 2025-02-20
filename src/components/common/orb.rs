use dioxus::prelude::*;

// #[component]
// pub fn Orb(
//     class: Option<String>,
//     is_gold: bool,
//     is_large: Option<bool>,
// ) -> Element {
//     let class = class.unwrap_or_default();
//     let animation = if is_gold { "animate-orb-spin-fast" } else { "animate-orb-spin-slow" };
//     let image = if is_gold { "bg-[url('assets/orb_gold_sprite.webp')]" } else { "bg-[url('assets/orb_black_sprite.webp')]" };
//     rsx! {
//         div {
//             class: "relative bg-no-repeat w-32 h-32 {image} {animation} {class}",
//         }
//     }
// }

#[component]
pub fn HeroOrb() -> Element {
    rsx! {
        div {
            class: "absolute z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-64 md:-right-64 lg:left-72 lg:-right-72 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // class: "fixed z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-56 md:-right-56 lg:left-64 lg:-right-64 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; overflow: visible;" 
                    url="https://prod.spline.design/Y2igVlaEL2MBaBOY/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingMiner() -> Element {
    rsx! {
        div {
            class: "absolute bottom-0 left-0 right-0 top-16 w-full h-full overflow-clipped pointer-events-none",
            // class: "absolute bottom-0 max-w-7xl mx-auto w-full h-full overflow-clipped pointer-events-none",
            // class: "absolute z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-64 md:-right-64 lg:left-72 lg:-right-72 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // class: "fixed z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-56 md:-right-56 lg:left-64 lg:-right-64 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; width: 100%;" 
                    url="https://prod.spline.design/uJNJJEEp0yCY0MvG/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingWave() -> Element {
    rsx! {
        div {
            class: "absolute bottom-0 left-0 md:left-auto max-w-7xl max-h-160 mx-auto w-full h-full overflow-clipped pointer-events-none",
            // class: "absolute z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-64 md:-right-64 lg:left-72 lg:-right-72 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // class: "fixed z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-56 md:-right-56 lg:left-64 lg:-right-64 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; width: 100%;" 
                    url="https://prod.spline.design/AUe5QgM7p99M8Cm1/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingOrbit() -> Element {
    rsx! {
        div {
            class: "w-full h-full overflow-clipped pointer-events-none",
            // class: "absolute z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-64 md:-right-64 lg:left-72 lg:-right-72 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // class: "fixed z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-56 md:-right-56 lg:left-64 lg:-right-64 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; width: 100%;" 
                    url="https://prod.spline.design/RGffAqOACmF8v29S/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn OrbMiner(class: Option<String>, gold: bool) -> Element {
    let class = class.unwrap_or_default();
    let asset_id = if gold { "Ow2sG0dhJar0f3VM" } else { "KM4FufZYWJA5RJ1a" };
    rsx! {
        div {
            class: "bg-transparent {class}",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 16rem; width: 16rem;" 
                    url="https://prod.spline.design/{asset_id}/scene.splinecode"
                />
            "#
        }
    }
}
