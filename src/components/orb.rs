use dioxus::prelude::*;

#[component]
pub fn Orb(
    class: Option<String>,
    is_gold: bool,
    is_large: Option<bool>,
) -> Element {
    let class = class.unwrap_or_default();
    let animation = if is_gold { "animate-orb-spin-fast" } else { "animate-orb-spin-slow" };
    let image = if is_gold { "bg-[url('assets/orb_gold_sprite.webp')]" } else { "bg-[url('assets/orb_black_sprite.webp')]" };
    rsx! {
        div {
            class: "relative bg-no-repeat w-32 h-32 {image} {animation} {class}",
        }
    }
}

pub fn OrbInteractive() -> Element {
    rsx! {
        div {
            class: "bg-transparent mx-auto",
            dangerous_inner_html: r#"
                <spline-viewer 
                    style="height: 12rem; width: 12rem;" 
                    url="https://prod.spline.design/zyaww3tr0AUmyWMP/scene.splinecode"
                />
            "#
        }
    }
}
