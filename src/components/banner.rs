use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum BannerType {
    Error,
}

#[derive(Props, PartialEq)]
pub struct BannerProps {
    pub text: String,
    pub banner_type: BannerType,
}

#[component]
pub fn Banner(cx: Scope<BannerProps>) -> Element {
    let color = match cx.props.banner_type {
        BannerType::Error => "bg-red-500 text-white",
    };
    render! {
        div {
            class: "flex flex-row w-full font-medium text-center gap-1 px-4 sm:px-8 py-2 text-sm {color}",
            // WarningIcon {
            //     class: "w-4 h-4 my-auto",
            // }
            p {
                "{cx.props.text}"
            }
        }
    }
}
