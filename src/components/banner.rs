use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BannerType {
    Error,
}

pub fn Banner(text: String, banner_type: BannerType) -> Element {
    let color = match banner_type {
        BannerType::Error => "bg-red-500 text-white",
    };
    rsx! {
        div {
            class: "flex flex-row w-full font-medium text-center gap-1 px-4 sm:px-8 py-2 text-sm {color}",
            // WarningIcon {
            //     class: "w-4 h-4 my-auto",
            // }
            p {
                "{text}"
            }
        }
    }
}
