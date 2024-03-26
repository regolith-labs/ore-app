use dioxus::prelude::*;

#[derive(Props)]
pub struct IconProps<'a> {
    pub class: Option<&'a str>,
    pub solid: Option<bool>,
}

// <svg width="216" height="216" viewBox="0 0 216 216" fill="none" xmlns="http://www.w3.org/2000/svg">
// <path fill-rule="evenodd" clip-rule="evenodd" d="M0.279729 192.083C-0.0932429 191.71 -0.0932429 191.105 0.279729 190.732L28.4516 162.56C28.7938 162.218 28.8414 161.68 28.5687 161.28C18.1262 145.969 12.0208 127.463 12.0208 107.532C12.0208 54.7824 54.7823 12.0209 107.531 12.0209C127.463 12.0209 145.969 18.1262 161.28 28.569C161.68 28.8417 162.218 28.7941 162.56 28.4519L190.732 0.279816C191.105 -0.0932721 191.71 -0.0932721 192.083 0.279816L215.72 23.9178C216.093 24.2908 216.093 24.8953 215.72 25.2683L187.365 53.6242C187.026 53.9626 186.975 54.493 187.239 54.8921C197.227 69.9845 203.042 88.0792 203.042 107.532C203.042 160.281 160.28 203.042 107.531 203.042C88.0788 203.042 69.9844 197.226 54.8921 187.24C54.4929 186.976 53.9625 187.026 53.6241 187.365L25.2681 215.721C24.8952 216.094 24.2904 216.094 23.9174 215.721L0.279729 192.083ZM107.531 167.703C97.5942 167.703 88.2198 165.294 79.96 161.029C69.2678 155.507 60.4434 146.875 54.6844 136.327C50.0141 127.774 47.3597 117.963 47.3597 107.532C47.3597 74.2996 74.2995 47.3598 107.531 47.3598C117.963 47.3598 127.774 50.0144 136.327 54.6845C146.874 60.4431 155.507 69.2685 161.029 79.9603C165.294 88.2205 167.703 97.5943 167.703 107.532C167.703 140.763 140.763 167.703 107.531 167.703Z" fill="#1D1D1F"/>
// </svg>

#[component]
pub fn OreIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            // view_box: "0 0 48 48",
            // view_box: "0 0 218 218",
            view_box: "0 0 216 216",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M0.279729 192.083C-0.0932429 191.71 -0.0932429 191.105 0.279729 190.732L28.4516 162.56C28.7938 162.218 28.8414 161.68 28.5687 161.28C18.1262 145.969 12.0208 127.463 12.0208 107.532C12.0208 54.7824 54.7823 12.0209 107.531 12.0209C127.463 12.0209 145.969 18.1262 161.28 28.569C161.68 28.8417 162.218 28.7941 162.56 28.4519L190.732 0.279816C191.105 -0.0932721 191.71 -0.0932721 192.083 0.279816L215.72 23.9178C216.093 24.2908 216.093 24.8953 215.72 25.2683L187.365 53.6242C187.026 53.9626 186.975 54.493 187.239 54.8921C197.227 69.9845 203.042 88.0792 203.042 107.532C203.042 160.281 160.28 203.042 107.531 203.042C88.0788 203.042 69.9844 197.226 54.8921 187.24C54.4929 186.976 53.9625 187.026 53.6241 187.365L25.2681 215.721C24.8952 216.094 24.2904 216.094 23.9174 215.721L0.279729 192.083ZM107.531 167.703C97.5942 167.703 88.2198 165.294 79.96 161.029C69.2678 155.507 60.4434 146.875 54.6844 136.327C50.0141 127.774 47.3597 117.963 47.3597 107.532C47.3597 74.2996 74.2995 47.3598 107.531 47.3598C117.963 47.3598 127.774 50.0144 136.327 54.6845C146.874 60.4431 155.507 69.2685 161.029 79.9603C165.294 88.2205 167.703 97.5943 167.703 107.532C167.703 140.763 140.763 167.703 107.531 167.703Z"
                // d: "M0.292923 192.626C-0.097641 192.235 -0.097641 191.603 0.292923 191.212L25.6202 165.884C25.9615 165.543 26.0099 165.007 25.7392 164.607C14.911 148.626 8.58583 129.345 8.58583 108.586C8.58583 53.3574 53.3573 8.58594 108.586 8.58594C129.344 8.58594 148.626 14.9114 164.608 25.739C165.007 26.0096 165.543 25.9612 165.884 25.6199L191.212 0.292969C191.602 -0.0976562 192.235 -0.0976562 192.626 0.292969L217.375 25.042C217.765 25.4326 217.765 26.0654 217.375 26.4561L191.95 51.88C191.611 52.2194 191.561 52.7516 191.827 53.1508C202.414 69.0175 208.586 88.0805 208.586 108.586C208.586 163.814 163.814 208.586 108.586 208.586C88.0807 208.586 69.0171 202.414 53.1511 191.827C52.7519 191.561 52.2197 191.611 51.8803 191.95L26.4559 217.375C26.0653 217.766 25.4321 217.766 25.0416 217.375L0.292923 192.626ZM108.586 171.586C98.0533 171.586 88.1245 169.001 79.3989 164.432C68.2422 158.589 59.0526 149.501 53.0841 138.421C48.3004 129.54 45.5858 119.38 45.5858 108.586C45.5858 73.792 73.7919 45.5859 108.586 45.5859C119.38 45.5859 129.54 48.3008 138.42 53.084C149.501 59.0527 158.589 68.2422 164.432 79.3994C169.001 88.124 171.586 98.0537 171.586 108.586C171.586 143.38 143.38 171.586 108.586 171.586Z",
                // d: "M0.00527625 42.1406C-0.010802 42.0669 0.00987002 41.9866 0.0672362 41.9293L4.91651 37.0801C4.9946 37.002 5.0059 36.8794 4.94428 36.7878C2.48729 33.1337 1.0535 28.7342 1.0535 23.9999C1.0535 11.3269 11.327 1.05343 24 1.05343C28.7343 1.05343 33.1338 2.48717 36.7878 4.94421C36.8794 5.00584 37.002 4.99454 37.0801 4.91645L41.9293 0.0672259C42.0189 -0.0224086 42.1642 -0.0224086 42.2538 0.0672259L47.9327 5.74625C48.0224 5.83588 48.0224 5.98109 47.9327 6.07073L43.0835 10.9199C43.0054 10.998 42.9941 11.1206 43.0557 11.2122C45.5127 14.8661 46.9464 19.2655 46.9464 23.9999C46.9464 36.6729 36.6729 46.9463 24 46.9463C19.2657 46.9463 14.8663 45.5126 11.2122 43.0556C11.1206 42.9939 10.998 43.0052 10.92 43.0833L6.07068 47.9328C5.9811 48.0224 5.83578 48.0224 5.7462 47.9328L0.0672362 42.2537C0.0349677 42.2215 0.0143518 42.182 0.00527625 42.1406ZM24 38.4562C21.5532 38.4562 19.2483 37.8482 17.2282 36.7753C14.6769 35.4202 12.5798 33.323 11.2247 30.7718C10.1516 28.7516 9.54369 26.4467 9.54369 23.9999C9.54369 16.0159 16.016 9.54362 24 9.54362C26.4469 9.54362 28.7518 10.1516 30.7719 11.2245C33.3232 12.5798 35.4202 14.6768 36.7753 17.228C37.8483 19.2481 38.4562 21.5531 38.4562 23.9999C38.4562 31.9839 31.9839 38.4562 24 38.4562Z"
            }
        }
    }
}

#[component]
pub fn QrCodeIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0 1 3.75 9.375v-4.5ZM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 0 1-1.125-1.125v-4.5ZM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0 1 13.5 9.375v-4.5Z"
            }
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6.75 6.75h.75v.75h-.75v-.75ZM6.75 16.5h.75v.75h-.75v-.75ZM16.5 6.75h.75v.75h-.75v-.75ZM13.5 13.5h.75v.75h-.75v-.75ZM13.5 19.5h.75v.75h-.75v-.75ZM19.5 13.5h.75v.75h-.75v-.75ZM19.5 19.5h.75v.75h-.75v-.75ZM16.5 16.5h.75v.75h-.75v-.75Z"
            }
        }
    }
}

#[component]
pub fn SearchIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            fill: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn CubeIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M12.378 1.602a.75.75 0 0 0-.756 0L3 6.632l9 5.25 9-5.25-8.622-5.03ZM21.75 7.93l-9 5.25v9l8.628-5.032a.75.75 0 0 0 .372-.648V7.93ZM11.25 22.18v-9l-9-5.25v8.57a.75.75 0 0 0 .372.648l8.628 5.033Z"
            }
        }
    }
}

#[component]
pub fn CubeTransparentIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M11.622 1.602a.75.75 0 0 1 .756 0l2.25 1.313a.75.75 0 0 1-.756 1.295L12 3.118 10.128 4.21a.75.75 0 1 1-.756-1.295l2.25-1.313ZM5.898 5.81a.75.75 0 0 1-.27 1.025l-1.14.665 1.14.665a.75.75 0 1 1-.756 1.295L3.75 8.806v.944a.75.75 0 0 1-1.5 0V7.5a.75.75 0 0 1 .372-.648l2.25-1.312a.75.75 0 0 1 1.026.27Zm12.204 0a.75.75 0 0 1 1.026-.27l2.25 1.312a.75.75 0 0 1 .372.648v2.25a.75.75 0 0 1-1.5 0v-.944l-1.122.654a.75.75 0 1 1-.756-1.295l1.14-.665-1.14-.665a.75.75 0 0 1-.27-1.025Zm-9 5.25a.75.75 0 0 1 1.026-.27L12 11.882l1.872-1.092a.75.75 0 1 1 .756 1.295l-1.878 1.096V15a.75.75 0 0 1-1.5 0v-1.82l-1.878-1.095a.75.75 0 0 1-.27-1.025ZM3 13.5a.75.75 0 0 1 .75.75v1.82l1.878 1.095a.75.75 0 1 1-.756 1.295l-2.25-1.312a.75.75 0 0 1-.372-.648v-2.25A.75.75 0 0 1 3 13.5Zm18 0a.75.75 0 0 1 .75.75v2.25a.75.75 0 0 1-.372.648l-2.25 1.312a.75.75 0 1 1-.756-1.295l1.878-1.096V14.25a.75.75 0 0 1 .75-.75Zm-9 5.25a.75.75 0 0 1 .75.75v.944l1.122-.654a.75.75 0 1 1 .756 1.295l-2.25 1.313a.75.75 0 0 1-.756 0l-2.25-1.313a.75.75 0 1 1 .756-1.295l1.122.654V19.5a.75.75 0 0 1 .75-.75Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn AdjustmentsHorizontalIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            stroke_width: 0,
            path {
                d: "M18.75 12.75h1.5a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM12 6a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 6ZM12 18a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 18ZM3.75 6.75h1.5a.75.75 0 1 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM5.25 18.75h-1.5a.75.75 0 0 1 0-1.5h1.5a.75.75 0 0 1 0 1.5ZM3 12a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 3 12ZM9 3.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5ZM12.75 12a2.25 2.25 0 1 1 4.5 0 2.25 2.25 0 0 1-4.5 0ZM9 15.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z"
            }
        }
    }
}

#[component]
pub fn PauseIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn PlayIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn PlusIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 20 20",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z"
            }
        }
    }
}

#[component]
pub fn InfoIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 16 16",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M15 8A7 7 0 1 1 1 8a7 7 0 0 1 14 0ZM9 5a1 1 0 1 1-2 0 1 1 0 0 1 2 0ZM6.75 8a.75.75 0 0 0 0 1.5h.75v1.75a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8.25 8h-1.5Z",
            }
        }
    }
}

#[component]
pub fn WarningIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003ZM12 8.25a.75.75 0 0 1 .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75Zm0 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
        }
    }
}

#[component]
pub fn CheckIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z"
            }
        }
    }
}

#[component]
pub fn UserIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.5 6a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM3.751 20.105a8.25 8.25 0 0 1 16.498 0 .75.75 0 0 1-.437.695A18.683 18.683 0 0 1 12 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 0 1-.437-.695Z"
            }
        }
    }
}

#[component]
pub fn UserGroupIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M8.25 6.75a3.75 3.75 0 1 1 7.5 0 3.75 3.75 0 0 1-7.5 0ZM15.75 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM2.25 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM6.31 15.117A6.745 6.745 0 0 1 12 12a6.745 6.745 0 0 1 6.709 7.498.75.75 0 0 1-.372.568A12.696 12.696 0 0 1 12 21.75c-2.305 0-4.47-.612-6.337-1.684a.75.75 0 0 1-.372-.568 6.787 6.787 0 0 1 1.019-4.38Z",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.082 14.254a8.287 8.287 0 0 0-1.308 5.135 9.687 9.687 0 0 1-1.764-.44l-.115-.04a.563.563 0 0 1-.373-.487l-.01-.121a3.75 3.75 0 0 1 3.57-4.047ZM20.226 19.389a8.287 8.287 0 0 0-1.308-5.135 3.75 3.75 0 0 1 3.57 4.047l-.01.121a.563.563 0 0 1-.373.486l-.115.04c-.567.2-1.156.349-1.764.441Z"
            }
        }
    }
}

// <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
//   <path fill-rule="evenodd" d="M8.25 6.75a3.75 3.75 0 1 1 7.5 0 3.75 3.75 0 0 1-7.5 0ZM15.75 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM2.25 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM6.31 15.117A6.745 6.745 0 0 1 12 12a6.745 6.745 0 0 1 6.709 7.498.75.75 0 0 1-.372.568A12.696 12.696 0 0 1 12 21.75c-2.305 0-4.47-.612-6.337-1.684a.75.75 0 0 1-.372-.568 6.787 6.787 0 0 1 1.019-4.38Z" clip-rule="evenodd" />
//   <path d="M5.082 14.254a8.287 8.287 0 0 0-1.308 5.135 9.687 9.687 0 0 1-1.764-.44l-.115-.04a.563.563 0 0 1-.373-.487l-.01-.121a3.75 3.75 0 0 1 3.57-4.047ZM20.226 19.389a8.287 8.287 0 0 0-1.308-5.135 3.75 3.75 0 0 1 3.57 4.047l-.01.121a.563.563 0 0 1-.373.486l-.115.04c-.567.2-1.156.349-1.764.441Z" />
// </svg>

#[component]
pub fn GlobeIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M21.721 12.752a9.711 9.711 0 0 0-.945-5.003 12.754 12.754 0 0 1-4.339 2.708 18.991 18.991 0 0 1-.214 4.772 17.165 17.165 0 0 0 5.498-2.477ZM14.634 15.55a17.324 17.324 0 0 0 .332-4.647c-.952.227-1.945.347-2.966.347-1.021 0-2.014-.12-2.966-.347a17.515 17.515 0 0 0 .332 4.647 17.385 17.385 0 0 0 5.268 0ZM9.772 17.119a18.963 18.963 0 0 0 4.456 0A17.182 17.182 0 0 1 12 21.724a17.18 17.18 0 0 1-2.228-4.605ZM7.777 15.23a18.87 18.87 0 0 1-.214-4.774 12.753 12.753 0 0 1-4.34-2.708 9.711 9.711 0 0 0-.944 5.004 17.165 17.165 0 0 0 5.498 2.477ZM21.356 14.752a9.765 9.765 0 0 1-7.478 6.817 18.64 18.64 0 0 0 1.988-4.718 18.627 18.627 0 0 0 5.49-2.098ZM2.644 14.752c1.682.971 3.53 1.688 5.49 2.099a18.64 18.64 0 0 0 1.988 4.718 9.765 9.765 0 0 1-7.478-6.816ZM13.878 2.43a9.755 9.755 0 0 1 6.116 3.986 11.267 11.267 0 0 1-3.746 2.504 18.63 18.63 0 0 0-2.37-6.49ZM12 2.276a17.152 17.152 0 0 1 2.805 7.121c-.897.23-1.837.353-2.805.353-.968 0-1.908-.122-2.805-.353A17.151 17.151 0 0 1 12 2.276ZM10.122 2.43a18.629 18.629 0 0 0-2.37 6.49 11.266 11.266 0 0 1-3.746-2.504 9.754 9.754 0 0 1 6.116-3.985Z"
            }
        }
    }
}

#[component]
pub fn CircleStackIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M21 6.375c0 2.692-4.03 4.875-9 4.875S3 9.067 3 6.375 7.03 1.5 12 1.5s9 2.183 9 4.875Z",
            }
            path {
                fill_rule: "evenodd",
                d: "M12 12.75c2.685 0 5.19-.586 7.078-1.609a8.283 8.283 0 0 0 1.897-1.384c.016.121.025.244.025.368C21 12.817 16.97 15 12 15s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.285 8.285 0 0 0 1.897 1.384C6.809 12.164 9.315 12.75 12 12.75Z",
            }
            path {
                fill_rule: "evenodd",
                d: "M12 16.5c2.685 0 5.19-.586 7.078-1.609a8.282 8.282 0 0 0 1.897-1.384c.016.121.025.244.025.368 0 2.692-4.03 4.875-9 4.875s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.284 8.284 0 0 0 1.897 1.384C6.809 15.914 9.315 16.5 12 16.5Z"
            }
            path {
                fill_rule: "evenodd",
                d: "M12 20.25c2.685 0 5.19-.586 7.078-1.609a8.282 8.282 0 0 0 1.897-1.384c.016.121.025.244.025.368 0 2.692-4.03 4.875-9 4.875s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.284 8.284 0 0 0 1.897 1.384C6.809 19.664 9.315 20.25 12 20.25Z"
            }
        }
    }
}

#[component]
pub fn PaperAirplaneIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M3.478 2.404a.75.75 0 0 0-.926.941l2.432 7.905H13.5a.75.75 0 0 1 0 1.5H4.984l-2.432 7.905a.75.75 0 0 0 .926.94 60.519 60.519 0 0 0 18.445-8.986.75.75 0 0 0 0-1.218A60.517 60.517 0 0 0 3.478 2.404Z"
            }
        }
    }
}

#[component]
pub fn CopyIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    if cx.props.solid.unwrap_or_default() {
        render! {
            svg {
                view_box: "0 0 24 24",
                fill: "currentColor",
                class: "{class}",
                path {
                    fill_rule: "evenodd",
                    d: "M7.5 3.375c0-1.036.84-1.875 1.875-1.875h.375a3.75 3.75 0 0 1 3.75 3.75v1.875C13.5 8.161 14.34 9 15.375 9h1.875A3.75 3.75 0 0 1 21 12.75v3.375C21 17.16 20.16 18 19.125 18h-9.75A1.875 1.875 0 0 1 7.5 16.125V3.375Z",
                }
                path {
                    fill_rule: "evenodd",
                    d: "M15 5.25a5.23 5.23 0 0 0-1.279-3.434 9.768 9.768 0 0 1 6.963 6.963A5.23 5.23 0 0 0 17.25 7.5h-1.875A.375.375 0 0 1 15 7.125V5.25ZM4.875 6H6v10.125A3.375 3.375 0 0 0 9.375 19.5H16.5v1.125c0 1.035-.84 1.875-1.875 1.875h-9.75A1.875 1.875 0 0 1 3 20.625V7.875C3 6.839 3.84 6 4.875 6Z",
                }
            }
        }
    } else {
        render! {
            svg {
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                class: "{class}",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
                }
            }
        }
    }
}

#[component]
pub fn ChartIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M18.375 2.25c-1.035 0-1.875.84-1.875 1.875v15.75c0 1.035.84 1.875 1.875 1.875h.75c1.035 0 1.875-.84 1.875-1.875V4.125c0-1.036-.84-1.875-1.875-1.875h-.75ZM9.75 8.625c0-1.036.84-1.875 1.875-1.875h.75c1.036 0 1.875.84 1.875 1.875v11.25c0 1.035-.84 1.875-1.875 1.875h-.75a1.875 1.875 0 0 1-1.875-1.875V8.625ZM3 13.125c0-1.036.84-1.875 1.875-1.875h.75c1.036 0 1.875.84 1.875 1.875v6.75c0 1.035-.84 1.875-1.875 1.875h-.75A1.875 1.875 0 0 1 3 19.875v-6.75Z"
            }
        }
    }
}

#[component]
pub fn GithubIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 1024 1024",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                transform: "scale(64)",
                d: "M8 0C3.58 0 0 3.58 0 8C0 11.54 2.29 14.53 5.47 15.59C5.87 15.66 6.02 15.42 6.02 15.21C6.02 15.02 6.01 14.39 6.01 13.72C4 14.09 3.48 13.23 3.32 12.78C3.23 12.55 2.84 11.84 2.5 11.65C2.22 11.5 1.82 11.13 2.49 11.12C3.12 11.11 3.57 11.7 3.72 11.94C4.44 13.15 5.59 12.81 6.05 12.6C6.12 12.08 6.33 11.73 6.56 11.53C4.78 11.33 2.92 10.64 2.92 7.58C2.92 6.71 3.23 5.99 3.74 5.43C3.66 5.23 3.38 4.41 3.82 3.31C3.82 3.31 4.49 3.1 6.02 4.13C6.66 3.95 7.34 3.86 8.02 3.86C8.7 3.86 9.38 3.95 10.02 4.13C11.55 3.09 12.22 3.31 12.22 3.31C12.66 4.41 12.38 5.23 12.3 5.43C12.81 5.99 13.12 6.7 13.12 7.58C13.12 10.65 11.25 11.33 9.47 11.53C9.76 11.78 10.01 12.26 10.01 13.01C10.01 14.08 10 14.94 10 15.21C10 15.42 10.15 15.67 10.55 15.59C13.71 14.53 16 11.53 16 8C16 3.58 12.42 0 8 0Z",
            }
        }
    }
}

#[component(no_case_check)]
pub fn XIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 300 300",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M178.57 127.15 290.27 0h-26.46l-97.03 110.38L89.34 0H0l117.13 166.93L0 300.25h26.46l102.4-116.59 81.8 116.59h89.34M36.01 19.54H76.66l187.13 262.13h-40.66"
            }
        }
    }
}

pub fn OreLogoIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 416 142",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M0.183896 126.277C-0.0612986 126.031 -0.0612986 125.634 0.183896 125.389L18.5017 107.071C18.8439 106.728 18.8913 106.191 18.6205 105.79C11.8537 95.7702 7.90257 83.6927 7.90257 70.692C7.90257 36.0144 36.0143 7.90263 70.692 7.90263C83.6925 7.90263 95.7702 11.8536 105.79 18.6206C106.191 18.8915 106.729 18.8441 107.071 18.5019L125.389 0.183953C125.634 -0.0613178 126.031 -0.0613178 126.276 0.183953L141.816 15.7237C142.061 15.969 142.061 16.3663 141.816 16.6116L123.374 35.0533C123.036 35.3917 122.986 35.922 123.248 36.3223C129.718 46.1966 133.481 58.0051 133.481 70.692C133.481 105.37 105.37 133.481 70.692 133.481C58.0048 133.481 46.1965 129.718 36.3223 123.248C35.9219 122.986 35.3917 123.036 35.0532 123.375L16.6115 141.816C16.3663 142.062 15.9687 142.062 15.7235 141.816L0.183896 126.277ZM70.692 110.249C64.1592 110.249 57.9963 108.665 52.5663 105.861C45.5372 102.231 39.736 96.5565 35.9499 89.6226C32.8797 83.9998 31.1346 77.5498 31.1346 70.692C31.1346 48.8451 48.8451 31.1347 70.692 31.1347C77.5497 31.1347 83.9997 32.8798 89.6225 35.95C96.5561 39.7357 102.231 45.5376 105.861 52.5665C108.666 57.9968 110.249 64.1592 110.249 70.692C110.249 92.5389 92.5389 110.249 70.692 110.249Z"
            }
            path {
                d: "M358.969 111.552V30.4483H415.668V48.0593H380.867V61.6538H414.042V79.149H380.867V93.9407H415.664V111.552H358.969Z"
            }
            path {
                d: "M282.189 111.552V30.4483H319.034C334.675 30.4483 347.767 41.8028 347.767 58.8345C347.767 70.5945 341.685 79.6897 332.589 84.2662L350.78 111.552H325.985L312.429 89.3062H304.087V111.552H282.189ZM314.863 50.4924H304.087V69.4938H314.863C322.394 69.4938 326.217 65.5545 326.217 59.8772C326.217 54.2 322.394 50.4924 314.863 50.4924Z"
            }
            path {
                d: "M232 113C206.083 113 190 92.7479 190 71C190 49.2521 206.083 29 232 29C257.917 29 274 49.2521 274 71C274 92.7479 257.917 113 232 113ZM232 91.8274C244.38 91.8274 252.017 81.9315 252.017 71C252.017 60.0685 244.38 50.1726 232 50.1726C219.62 50.1726 211.983 60.0685 211.983 71C211.983 81.9315 219.62 91.8274 232 91.8274Z"
            }
        }
    }
}

pub fn OreWordmarkIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 234 88",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M175.14 86V2H233.864V20.24H197.82V34.32H232.18V52.44H197.82V67.76H233.86V86H175.14Z"
            }
            path {
                d: "M95.618 86V2H133.778C149.978 2 163.538 13.76 163.538 31.4C163.538 43.58 157.238 53 147.818 57.74L166.658 86H140.978L126.938 62.96H118.298V86H95.618ZM129.458 22.76H118.298V42.44H129.458C137.258 42.44 141.218 38.36 141.218 32.48C141.218 26.6 137.258 22.76 129.458 22.76Z"
            }
            path {
                d: "M43.6361 87.5C16.7931 87.5 0.136078 66.5247 0.136078 44C0.136078 21.4753 16.7931 0.5 43.6361 0.5C70.4791 0.5 87.1361 21.4753 87.1361 44C87.1361 66.5247 70.4791 87.5 43.6361 87.5ZM43.6361 65.5712C56.4584 65.5712 64.3675 55.3219 64.3675 44C64.3675 32.6781 56.4584 22.4288 43.6361 22.4288C30.8138 22.4288 22.9047 32.6781 22.9047 44C22.9047 55.3219 30.8138 65.5712 43.6361 65.5712Z"
            }
        }
    }
}

pub fn TreasuryIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M11.584 2.376a.75.75 0 0 1 .832 0l9 6a.75.75 0 1 1-.832 1.248L12 3.901 3.416 9.624a.75.75 0 0 1-.832-1.248l9-6Z"
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M20.25 10.332v9.918H21a.75.75 0 0 1 0 1.5H3a.75.75 0 0 1 0-1.5h.75v-9.918a.75.75 0 0 1 .634-.74A49.109 49.109 0 0 1 12 9c2.59 0 5.134.202 7.616.592a.75.75 0 0 1 .634.74Zm-7.5 2.418a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Zm3-.75a.75.75 0 0 1 .75.75v6.75a.75.75 0 0 1-1.5 0v-6.75a.75.75 0 0 1 .75-.75ZM9 12.75a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Z"
            }
            path {
                d: "M12 7.875a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z"
            }
        }
    }
}

pub fn BusIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M3.375 4.5C2.339 4.5 1.5 5.34 1.5 6.375V13.5h12V6.375c0-1.036-.84-1.875-1.875-1.875h-8.25ZM13.5 15h-12v2.625c0 1.035.84 1.875 1.875 1.875h.375a3 3 0 1 1 6 0h3a.75.75 0 0 0 .75-.75V15Z"
            }
            path {
                d: "M8.25 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0ZM15.75 6.75a.75.75 0 0 0-.75.75v11.25c0 .087.015.17.042.248a3 3 0 0 1 5.958.464c.853-.175 1.522-.935 1.464-1.883a18.659 18.659 0 0 0-3.732-10.104 1.837 1.837 0 0 0-1.47-.725H15.75Z"
            }
            path {
                d: "M19.5 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z"
            }
        }
    }
}

pub fn PieIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 13.5a8.25 8.25 0 0 1 8.25-8.25.75.75 0 0 1 .75.75v6.75H18a.75.75 0 0 1 .75.75 8.25 8.25 0 0 1-16.5 0Z"
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M12.75 3a.75.75 0 0 1 .75-.75 8.25 8.25 0 0 1 8.25 8.25.75.75 0 0 1-.75.75h-7.5a.75.75 0 0 1-.75-.75V3Z"
            }
        }
    }
}

// <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
//   <path fill-rule="evenodd" d="M2.25 13.5a8.25 8.25 0 0 1 8.25-8.25.75.75 0 0 1 .75.75v6.75H18a.75.75 0 0 1 .75.75 8.25 8.25 0 0 1-16.5 0Z" clip-rule="evenodd" />
//   <path fill-rule="evenodd" d="M12.75 3a.75.75 0 0 1 .75-.75 8.25 8.25 0 0 1 8.25 8.25.75.75 0 0 1-.75.75h-7.5a.75.75 0 0 1-.75-.75V3Z" clip-rule="evenodd" />
// </svg>

// <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
//   <path d="M3.375 4.5C2.339 4.5 1.5 5.34 1.5 6.375V13.5h12V6.375c0-1.036-.84-1.875-1.875-1.875h-8.25ZM13.5 15h-12v2.625c0 1.035.84 1.875 1.875 1.875h.375a3 3 0 1 1 6 0h3a.75.75 0 0 0 .75-.75V15Z" />
//   <path d="M8.25 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0ZM15.75 6.75a.75.75 0 0 0-.75.75v11.25c0 .087.015.17.042.248a3 3 0 0 1 5.958.464c.853-.175 1.522-.935 1.464-1.883a18.659 18.659 0 0 0-3.732-10.104 1.837 1.837 0 0 0-1.47-.725H15.75Z" />
//   <path d="M19.5 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z" />
// </svg>

// <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
//   <path d="M11.584 2.376a.75.75 0 0 1 .832 0l9 6a.75.75 0 1 1-.832 1.248L12 3.901 3.416 9.624a.75.75 0 0 1-.832-1.248l9-6Z" />
//   <path fill-rule="evenodd" d="M20.25 10.332v9.918H21a.75.75 0 0 1 0 1.5H3a.75.75 0 0 1 0-1.5h.75v-9.918a.75.75 0 0 1 .634-.74A49.109 49.109 0 0 1 12 9c2.59 0 5.134.202 7.616.592a.75.75 0 0 1 .634.74Zm-7.5 2.418a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Zm3-.75a.75.75 0 0 1 .75.75v6.75a.75.75 0 0 1-1.5 0v-6.75a.75.75 0 0 1 .75-.75ZM9 12.75a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Z" clip-rule="evenodd" />
//   <path d="M12 7.875a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z" />
// </svg>

// <svg width="234" height="88" viewBox="0 0 234 88" fill="none" xmlns="http://www.w3.org/2000/svg">
// <path d="M175.14 86V2H233.864V20.24H197.82V34.32H232.18V52.44H197.82V67.76H233.86V86H175.14Z" fill="#1D1D1F"/>
// <path d="M95.618 86V2H133.778C149.978 2 163.538 13.76 163.538 31.4C163.538 43.58 157.238 53 147.818 57.74L166.658 86H140.978L126.938 62.96H118.298V86H95.618ZM129.458 22.76H118.298V42.44H129.458C137.258 42.44 141.218 38.36 141.218 32.48C141.218 26.6 137.258 22.76 129.458 22.76Z" fill="#1D1D1F"/>
// <path d="M43.6361 87.5C16.7931 87.5 0.136078 66.5247 0.136078 44C0.136078 21.4753 16.7931 0.5 43.6361 0.5C70.4791 0.5 87.1361 21.4753 87.1361 44C87.1361 66.5247 70.4791 87.5 43.6361 87.5ZM43.6361 65.5712C56.4584 65.5712 64.3675 55.3219 64.3675 44C64.3675 32.6781 56.4584 22.4288 43.6361 22.4288C30.8138 22.4288 22.9047 32.6781 22.9047 44C22.9047 55.3219 30.8138 65.5712 43.6361 65.5712Z" fill="#1D1D1F"/>
// </svg>

// <svg width="416" height="142" viewBox="0 0 416 142" fill="none" xmlns="http://www.w3.org/2000/svg">
// <path fill-rule="evenodd" clip-rule="evenodd" d="M0.356988 125.663C0.102196 125.409 0.102196 124.996 0.356988 124.741L16.8797 108.218C17.1024 107.995 17.134 107.645 16.9574 107.385C9.89339 96.9593 5.76703 84.3807 5.76703 70.8382C5.76703 34.8088 34.9746 5.60121 71.004 5.60121C84.5463 5.60121 97.1249 9.72777 107.551 16.7913C107.812 16.9679 108.161 16.9363 108.384 16.7137L124.907 0.191124C125.161 -0.063708 125.574 -0.063708 125.829 0.191124L141.975 16.3366C142.229 16.5915 142.229 17.0043 141.975 17.2591L125.388 33.845C125.167 34.0663 125.134 34.4135 125.308 34.674C132.215 45.025 136.241 57.4611 136.241 70.8382C136.241 106.868 107.033 136.075 71.004 136.075C57.6271 136.075 45.1906 132.049 34.8401 125.142C34.5796 124.968 34.2324 125.001 34.011 125.222L17.4249 141.809C17.1701 142.064 16.7571 142.064 16.5023 141.809L0.356988 125.663ZM71.004 111.937C64.1329 111.937 57.6556 110.251 51.9634 107.27C44.685 103.459 38.69 97.5299 34.7964 90.3016C31.6756 84.508 29.9047 77.8798 29.9047 70.8382C29.9047 48.1397 48.3055 29.7389 71.004 29.7389C78.0457 29.7389 84.6738 31.51 90.4672 34.6304C97.6956 38.5242 103.624 44.5191 107.436 51.7978C110.417 57.4894 112.103 63.9673 112.103 70.8382C112.103 93.5367 93.7025 111.937 71.004 111.937Z" fill="#1D1D1F"/>
// <path d="M359.135 111.552V30.4483H415.834V48.0593H381.033V61.6538H414.208V79.149H381.033V93.9407H415.83V111.552H359.135Z" fill="#1D1D1F"/>
// <path d="M282.355 111.552V30.4483H319.199C334.84 30.4483 347.933 41.8028 347.933 58.8345C347.933 70.5945 341.85 79.6897 332.755 84.2662L350.945 111.552H326.151L312.595 89.3062H304.253V111.552H282.355ZM315.028 50.4924H304.253V69.4938H315.028C322.559 69.4938 326.383 65.5545 326.383 59.8772C326.383 54.2 322.559 50.4924 315.028 50.4924Z" fill="#1D1D1F"/>
// <path d="M232.166 113C206.248 113 190.166 92.7479 190.166 71C190.166 49.2521 206.248 29 232.166 29C258.083 29 274.166 49.2521 274.166 71C274.166 92.7479 258.083 113 232.166 113ZM232.166 91.8274C244.546 91.8274 252.182 81.9315 252.182 71C252.182 60.0685 244.546 50.1726 232.166 50.1726C219.785 50.1726 212.149 60.0685 212.149 71C212.149 81.9315 219.785 91.8274 232.166 91.8274Z" fill="#1D1D1F"/>
// </svg>
