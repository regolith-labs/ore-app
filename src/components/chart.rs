use dioxus::prelude::*;

#[derive(Clone, Copy, Debug)]
// pub struct DataPoint<T: Ord + Display + Clone + Copy> {
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    // Add other properties as needed
}

#[derive(Props)]
// pub struct ChartProps<'a, T: Ord + Display + Clone + Copy> {
pub struct ChartProps<'a> {
    pub class: Option<&'a str>,
    // pub data: Vec<DataPoint<T>>,
    pub data: Vec<DataPoint>,
    pub height: f64,
    pub width: f64,
}

#[component]
// pub fn Chart<'a, T: Ord + Display + Clone + Copy>(cx: Scope<'a, ChartProps<'a, T>>) -> Element {
pub fn Chart<'a>(cx: Scope<'a, ChartProps<'a>>) -> Element {
    let highlighted_point = use_state::<Option<DataPoint>>(cx, || None);
    let class = cx.props.class.unwrap_or("");
    let data = cx.props.data.clone();
    let height = cx.props.height;
    let width = cx.props.width;
    let axis_width = 0.5;
    let axis_opacity = 0.5;

    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;
    data.iter().for_each(|d| {
        if d.x.gt(&max_x) {
            max_x = d.x;
        }
        if d.y.gt(&max_y) {
            max_y = d.y;
        }
        if d.x.lt(&min_x) {
            min_x = d.x;
        }
        if d.y.lt(&min_y) {
            min_y = d.y;
        }
    });

    // [min_x, max_x] => [0, width]
    // [min_y, max_y] => [0, height]
    // x: ((d.x - min_x) * width) / (max_x - min_x),
    // y: ((d.y - min_y) * height) / (max_y - min_y),
    let mapped_data = data
        .iter()
        .map(|d| DataPoint {
            x: ((d.x - min_x) * width) / (max_x - min_x),
            y: height - (((d.y - min_y) * height) / (max_y - min_y)),
            // x: map_x(d.x, min_x, max_x, width),
            // y: map_y(d.y, min_y, max_y, height),
        })
        .collect::<Vec<DataPoint>>();

    let path_data = mapped_data
        .iter()
        .enumerate()
        .fold(String::new(), |acc, (i, point)| {
            if i == 0 {
                format!("M{},{}", point.x, point.y)
            } else {
                format!("{} L{},{}", acc, point.x, point.y)
            }
        });

    let handle_mouse_move = move |event: MouseEvent| {
        let x = event.data.client_coordinates().x;
        let data = mapped_data.clone();
        let highlighted_point = highlighted_point.clone();
        async move {
            let mut closest_point: Option<DataPoint> = None;
            data.iter().for_each(|d| {
                if let Some(p) = closest_point {
                    if (d.x - x).abs() < (p.x - x).abs() {
                        closest_point = Some(*d);
                    }
                } else {
                    closest_point = Some(*d);
                }
            });
            highlighted_point.set(closest_point);
        }
    };

    let handle_mouse_leave = move |_e: MouseEvent| {
        highlighted_point.set(None);
        async move {}
    };

    render! {
        svg {
            class: "{class}",
            onmousemove: handle_mouse_move,
            onmouseleave: handle_mouse_leave,
            view_box: "-4 -8 {width + 4.0} {height + 16.0}",
            preserve_aspect_ratio: "xMidYMid meet",

            // Grid lines
            line {
                x1: "{0}",
                y1: "{height}",
                x2: "{width}",
                y2: "{height}",
                stroke: "currentColor",
                stroke_width: "{axis_width}",
                opacity: "{axis_opacity}"
            }
            line {
                x1: "{0}",
                y1: "{height * 0.666}",
                x2: "{width}",
                y2: "{height * 0.666}",
                stroke: "currentColor",
                stroke_width: "{axis_width}",
                opacity: "{axis_opacity}"
            }
            line {
                x1: "{0}",
                y1: "{height * 0.333}",
                x2: "{width}",
                y2: "{height * 0.333}",
                stroke: "currentColor",
                stroke_width: "{axis_width}",
                opacity: "{axis_opacity}"
            }
            line {
                x1: "{0}",
                y1: "{0f64}",
                x2: "{width}",
                y2: "{0f64}",
                stroke: "currentColor",
                stroke_width: "{axis_width}",
                opacity: "{axis_opacity}"
            }

            // Line data
            path {
                d: "{path_data}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
            }

            // Dot at highlighted point
            if let Some(p) = *highlighted_point.get() {
                render! {
                    circle {
                        cx: "{p.x}",
                        cy: "{p.y}",
                        r: "5",
                        fill: "currentColor",
                    }
                    // text {
                    //     fill: "currentColor",
                    //     x: "{p.x + 5.0}", // Offset tooltip a bit to the right
                    //     y: "{p.y}",
                    //     "{p.y}"
                    // }
                }
            }
        }
    }
}
