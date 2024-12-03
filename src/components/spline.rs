use dioxus::prelude::*;

pub fn SplineModel() -> Element {
    rsx! {
        div {
            class: "bg-transparent mx-auto",
            dangerous_inner_html: r#"
                <spline-viewer 
                    style="height: 32rem; width: 32rem;" 
                    url="https://prod.spline.design/KCMBCfI59-me0kTJ/scene.splinecode"
                />
            "#
        }
    }
}
