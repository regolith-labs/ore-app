use dioxus::prelude::*;

use crate::components::{Chart, DataPoint};

#[component]
pub fn MinerCharts(cx: Scope) -> Element {
    // let width = use_window_width(cx);
    let width = 0;

    // TODO Fetch data
    let data = vec![
        DataPoint { x: 10.0, y: 20.0 },
        DataPoint { x: 50.0, y: 60.0 },
        DataPoint { x: 70.0, y: 40.0 },
    ];

    render! {
        div {
            class: "flex h-full",
            Chart {
                class: "h-48 w-full",
                data: data,
                width: (width - 32) as f64,
                height: 192f64,
            }
        }
    }
}
