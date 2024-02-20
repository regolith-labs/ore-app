use dioxus::prelude::*;

use crate::components::InfoIcon;

pub enum TooltipDirection {
    Right,
    // Left,
    // Center,
}

#[derive(Props)]
pub struct TooltipProps<'a> {
    text: &'a str,
    direction: TooltipDirection,
}

pub fn Tooltip<'a>(cx: Scope<'a, TooltipProps<'a>>) -> Element<'a> {
    let info_icon_class = "w-3 h-3 my-auto opacity-80";
    let direction_class = match cx.props.direction {
        TooltipDirection::Right => "left-[50%]",
        // TooltipDirection::Left => "right-[50%]",
        // TooltipDirection::Center => "left-[50%] -translate-x-1/2",
    };
    render! {
        div {
            class: "relative flex group overflow-visible",
            InfoIcon {
                class: "{info_icon_class}"
            }
            span {
                class: "bottom-[100%] z-[100] absolute hidden group-hover:block w-max max-w-44 bg-gray-100 drop-shadow rounded py-1.5 px-2.5 text-xs text-black {direction_class}",
                "{cx.props.text}"
            }
        }
    }
}
