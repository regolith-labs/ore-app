use dioxus::prelude::*;

#[component]
pub fn HeroOrb() -> Element {
    rsx! {
        div {
            class: "w-full h-80 mt-auto",
            // class: "absolute z-40 -top-32 bottom-32 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-64 md:-right-64 lg:left-72 lg:-right-72 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // class: "fixed z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-56 md:-right-56 lg:left-64 lg:-right-64 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // https://prod.spline.design/Y2igVlaEL2MBaBOY/scene.splinecode
            // https://prod.spline.design/Ow2sG0dhJar0f3VM/scene.splinecode
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; overflow: visible;" 
                    url="https://prod.spline.design/jm1GUfqRPJx3ZY-V/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingGlobe() -> Element {
    rsx! {
        div {
            // class: "h-[60rem] w-full my-auto",
            // class: "absolute top-96 -bottom-96 md:top-16 md:-bottom-16 right-0 left-0 md:left-64 md:-right-64",
            class: "absolute top-128 -bottom-128 md:top-16 md:-bottom-16 right-0 left-0 md:left-80 md:-right-80",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; overflow: visible;" 
                    url="https://prod.spline.design/eznQUYBmCcX8LG3F/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingCoins() -> Element {
    rsx! {
        div {
            class: "absolute top-0 bottom-0 right-0 left-0",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; overflow: visible;" 
                    url="https://prod.spline.design/WQCasni25hk5LUV9/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingChain() -> Element {
    rsx! {
        div {
            class: "absolute top-0 bottom-0 right-0 left-0",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; overflow: visible;" 
                    url="https://prod.spline.design/LeWZHu2bcsgqDxXj/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn HeroBg() -> Element {
    rsx! {
        div {
            class: "absolute top-0 left-0 right-0 bottom-0",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; overflow: visible;"
                    url="https://prod.spline.design/KhvpG2ZZ2ELOEVO9/scene.splinecode"
                />
            "#
        }
    }
}

#[component]
pub fn LandingMiner() -> Element {
    rsx! {
        div {
            class: "h-160 w-screen md:w-lg lg:w-xl overflow-hidden shrink-0 pointer-events-none",
            // class: "absolute md:static md:my-auto h-160 bottom-0 md:bottom-auto -translate-x-1/2 md:translate-x-0 left-1/2 md:left-auto w-5xl md:w-lg lg:w-xl overflow-hidden shrink-0 pointer-events-none",
            // class: "absolute md:static h-160 w-5xl md:w-3xl -translate-x-1/2 bottom-0 md:bottom-auto md:translate-x-0 overflow-hidden shrink-0 pointer-events-none bg-red-500",
            // class: "absolute h-160 w-7xl -translate-x-1/2 left-1/2 bottom-0 overflow-hidden pointer-events-none",
            // class: "absolute bottom-0 left-0 right-0 top-16 w-full h-full overflow-clipped pointer-events-none",
            // class: "absolute bottom-0 max-w-7xl mx-auto w-full h-full overflow-clipped pointer-events-none",
            // class: "absolute z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-64 md:-right-64 lg:left-72 lg:-right-72 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // class: "fixed z-40 -top-48 bottom-48 left-0 right-0 sm:-top-32 sm:bottom-32 md:top-0 md:bottom-0 md:left-56 md:-right-56 lg:left-64 lg:-right-64 xl:left-80 xl:-right-80 bg-transparent overflow-visible pointer-events-none",
            // https://prod.spline.design/uJNJJEEp0yCY0MvG/scene.splinecode
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; width: 100%;" 
                    url="https://prod.spline.design/M54opX84FI0VNGE2/scene.splinecode"
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
    let asset_id = if gold {
        "Ow2sG0dhJar0f3VM"
    } else {
        "KM4FufZYWJA5RJ1a"
    };
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
