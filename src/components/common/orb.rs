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
pub fn OrbHero() -> Element {
    rsx! {
        // div {
        //     class: "bg-transparent mx-auto",
        //     dangerous_inner_html: r#"
        //         <spline-viewer
        //             style="height: 12rem; width: 12rem;" 
        //             url="https://prod.spline.design/zyaww3tr0AUmyWMP/scene.splinecode"
        //         />
        //     "#
        // }
        div {
            class: "absolute top-0 right-0 bottom-0 w-3/5",
            dangerous_inner_html: r#"
                <spline-viewer
                    style="height: 100%; width: 100%;" 
                    url="https://prod.spline.design/lIB-o0ilow1By5Yr/scene.splinecode"
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
