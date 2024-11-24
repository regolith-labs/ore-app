use dioxus::prelude::*;
#[component]
pub fn Row(class: Option<String>, children: Element, gap: Option<u8>, justify: Option<Justify>) -> Element {
    let class = class.unwrap_or("".to_string());
    let gap = gap.map_or("".to_string(), |g| format!("gap-{}", g));
    rsx! {
        Space {
            class: "flex flex-row {gap} {class}",
            justify: justify,
            {children}
        }
    }
}

#[component]
pub fn Col(class: Option<String>, children: Element, gap: Option<u8>, justify: Option<Justify>) -> Element {
    let class = class.unwrap_or("".to_string());
    let gap = gap.map_or("".to_string(), |g| format!("gap-{}", g));
    rsx! {
        Space {
            class: "flex flex-col {gap} {class}",
            justify: justify,
            {children}
        }
    }
}

#[component]
pub fn Space(
    children: Element,
    class: Option<String>,
    font_size: Option<FontSize>,
    font_weight: Option<FontWeight>,
    justify: Option<Justify>,
    text_align: Option<TextAlign>,
    text_color: Option<TextColor>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let font_size = match font_size {
        Some(FontSize::XS) => "text-xs",
        Some(FontSize::SM) => "text-sm",
        Some(FontSize::Base) => "text-base",
        Some(FontSize::LG) => "text-lg", 
        Some(FontSize::XL) => "text-xl",
        Some(FontSize::_2XL) => "text-2xl",
        Some(FontSize::_3XL) => "text-3xl",
        Some(FontSize::_4XL) => "text-4xl",
        Some(FontSize::_5XL) => "text-5xl",
        Some(FontSize::_6XL) => "text-6xl",
        Some(FontSize::_7XL) => "text-7xl",
        Some(FontSize::_8XL) => "text-8xl",
        Some(FontSize::_9XL) => "text-9xl",
        None => "",
    };
    let font_weight = match font_weight {
        Some(FontWeight::Thin) => "font-thin",
        Some(FontWeight::ExtraLight) => "font-extralight", 
        Some(FontWeight::Light) => "font-light",
        Some(FontWeight::Normal) => "font-normal",
        Some(FontWeight::Medium) => "font-medium",
        Some(FontWeight::SemiBold) => "font-semibold",
        Some(FontWeight::Bold) => "font-bold",
        Some(FontWeight::ExtraBold) => "font-extrabold",
        Some(FontWeight::Black) => "font-black",
        None => "",
    };
    let justify = match justify {
        Some(Justify::Around) => "justify-around",
        Some(Justify::Between) => "justify-between",
        Some(Justify::Center) => "justify-center",
        Some(Justify::End) => "justify-end",
        Some(Justify::Evenly) => "justify-evenly",
        Some(Justify::Normal) => "justify-normal",
        Some(Justify::Start) => "justify-start",
        Some(Justify::Stretch) => "justify-stretch",
        None => "",
    };
    let text_align = match text_align {
        Some(TextAlign::Left) => "text-left",
        Some(TextAlign::Center) => "text-center",
        Some(TextAlign::Right) => "text-right",
        Some(TextAlign::Justify) => "text-justify",
        Some(TextAlign::Start) => "text-start",
        Some(TextAlign::End) => "text-end",
        None => "",
    };
    let text_color = match text_color {
        Some(TextColor::LowEmphasis) => "text-elements-lowEmphasis",
        Some(TextColor::MidEmphasis) => "text-elements-midEmphasis",
        Some(TextColor::HighEmphasis) => "text-elements-highEmphasis",
        Some(TextColor::Gold) => "text-elements-gold",
        None => "",
    };
    rsx! {
        div {
            class: "{class} {font_size} {font_weight} {justify} {text_align} {text_color}",
            {children}
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum FontSize {
    XS,
    SM,
    Base,
    LG,
    XL,
    _2XL,
    _3XL,
    _4XL,
    _5XL,
    _6XL,
    _7XL,
    _8XL,
    _9XL,
}

#[derive(Clone, Eq, PartialEq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light, 
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Justify {
    Around,
    Between,
    Center,
    End,
    Evenly,
    Normal,
    Start,
    Stretch,
}

#[derive(Clone, Eq, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
}

#[derive(Clone, Eq, PartialEq)]
pub enum TextColor {
    LowEmphasis,
    MidEmphasis,
    HighEmphasis,
    Gold,
}