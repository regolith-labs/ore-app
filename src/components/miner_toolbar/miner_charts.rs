use dioxus::prelude::*;

use crate::components::{Chart, DataPoint};

use super::MinerChart;

#[derive(Props)]
pub struct MinerChartsProps<'a> {
    pub chart: &'a UseState<MinerChart>,
}

#[component]
pub fn MinerCharts<'a>(cx: Scope<'a, MinerChartsProps<'a>>) -> Element {
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
            match cx.props.chart.get() {
                MinerChart::Hash => {
                    render! {
                        p { "*hash chart*" }
                    }
                }
                MinerChart::Time => {
                    render! {
                        Chart {
                            class: "h-48 w-full",
                            data: data,
                            width: (width - 32) as f64,
                            height: 192f64,
                        }
                    }
                }
                MinerChart::Rewards => {
                    render! {
                        p { "*rewards chart*" }
                    }
                }
                MinerChart::Rate => {
                    render! {
                        p { "*rate chart*" }
                    }
                }
                MinerChart::Circulating => {
                    render! {
                        p { "*circulating chart*" }
                    }
                }
                MinerChart::Supply => {
                    render! {
                        p { "*supply chart*" }
                    }
                }
            }
        }
    }
}
