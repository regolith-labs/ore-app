use dioxus::prelude::*;

#[component]
pub fn MinerPower(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-8 mt-auto",
            div {
                class: "flex flex-col gap-2",
                h2 {
                    class: "text-2xl text-white font-bold",
                    "Power"
                }
                p {
                    class: "text-lg",
                    "Select how much of your computer should be dedicated to mining."
                }
                p {
                    class: "text-sm text-white opacity-80",
                    "Higher power levels will earn more rewards but may impact your computer's battery and/or performance."
                }
            }
            PowerBar {}
        }
    }
}

#[component]
pub fn PowerBar(cx: Scope) -> Element {
    let power_level = 4;
    render! {
        div {
            class: "flex flex-row gap-1 w-full",
            PowerBarLevel { id: 0, power_level: power_level }
            PowerBarLevel { id: 1, power_level: power_level }
            PowerBarLevel { id: 2, power_level: power_level }
            PowerBarLevel { id: 3, power_level: power_level }
            PowerBarLevel { id: 4, power_level: power_level }
            PowerBarLevel { id: 5, power_level: power_level }
            PowerBarLevel { id: 6, power_level: power_level }
            PowerBarLevel { id: 7, power_level: power_level }
        }
    }
}

#[component]
pub fn PowerBarLevel(cx: Scope, id: u8, power_level: u8) -> Element {
    let rounded = if id.eq(&0) {
        "rounded-l-full"
    } else if id.eq(&7) {
        "rounded-r-full"
    } else {
        ""
    };
    let opacity = if id.le(power_level) {
        ""
    } else {
        "opacity-40 hover:opacity-60"
    };
    render! {
        button {
            class: "w-full h-4 bg-white transition-opacity {rounded} {opacity}",
        }
    }
}
