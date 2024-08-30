use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum BannerStyle {
    Info,
    Error,
}

#[component]
pub fn Banner(style: BannerStyle, link_to: Option<String>, children: Element) -> Element {
    let color = match style {
        BannerStyle::Info => "bg-blue-500 text-white",
        BannerStyle::Error => "bg-red-500 text-white",
    };
    rsx! {
        if let Some(link_to) = link_to {
            Link {
                class: "flex flex-row w-full font-medium text-center gap-1 px-4 sm:px-8 py-2 text-sm {color}",
                to: link_to,
                {children}
            }
        } else {
            div {
                class: "flex flex-row w-full font-medium text-center gap-1 px-4 sm:px-8 py-2 text-xs {color}",
                {children}
            }
        }
    }
}
