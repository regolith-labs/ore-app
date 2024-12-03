use dioxus::prelude::*;

pub fn SplineModel() -> Element {
    rsx! {
        div {
            class: "bg-transparent mx-auto",
            dangerous_inner_html: r#"
                <spline-viewer 
                    style="height: 12rem; width: 12rem;" 
                    url="https://prod.spline.design/FEOrs1ySmEDKhU71/scene.splinecode"
                />
            "#
        }
    }
}
