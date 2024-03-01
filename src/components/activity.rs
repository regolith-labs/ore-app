use dioxus_router::components::Link;
#[cfg(feature = "desktop")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[cfg(feature = "web")]
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use ore_types::{Transfer, TransferType};

use crate::{
    components::{
        icons::CubeIcon, CircleStackIcon, GlobeIcon, OreIcon, PaperAirplaneIcon, UserIcon,
    },
    gateway::AsyncResult,
    hooks::{use_transfers, ACTIVITY_TABLE_PAGE_LIMIT},
};

#[derive(Debug)]
pub enum ActivityFilter {
    Global,
    Personal,
}

#[component]
pub fn Activity(cx: Scope) -> Element {
    let filter = use_state(cx, || ActivityFilter::Global);
    let offset = use_state(cx, || 0u64);
    let (transfers, has_more) = use_transfers(cx, filter, offset);
    match transfers {
        AsyncResult::Ok(transfers) => {
            render! {
                div {
                    class: "flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start",
                    div {
                        class: "flex flex-row justify-between",
                        h2 {
                            class: "text-lg md:text-2xl font-bold",
                            "Activity"
                        }
                        FilterButtons {
                            filter: filter,
                            offset: offset
                        }
                    }
                    ActivityTable{
                        offset: offset,
                        transfers: transfers,
                        has_more: has_more
                    }
                }
            }
        }
        _ => {
            render! {
                div {
                    class: "flex flex-row h-64 w-full loading rounded",
                }
            }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FilterButtonsProps<'a> {
    pub filter: &'a UseState<ActivityFilter>,
    pub offset: &'a UseState<u64>,
}

#[component]
pub fn FilterButtons<'a>(cx: Scope<'a, FilterButtonsProps<'a>>) -> Element {
    let offset = cx.props.offset;
    let filter = cx.props.filter.clone();
    let filter_ = cx.props.filter;
    let selected_class = "";
    let unselected_class = "text-gray-300 dark:text-gray-700";
    let (global_class, personal_class) = match filter.get() {
        ActivityFilter::Global => (selected_class, unselected_class),
        ActivityFilter::Personal => (unselected_class, selected_class),
    };
    let button_class =
        "flex flex-row gap-2 px-3 py-2 rounded-full text-sm hover-100 active-200 transition-colors";
    render! {
        div {
            // class: "flex flex-row gap-2 font-semibold -mx-4",
            class: "flex flex-row gap-2 font-semibold -mx-2",
            button {
                class: "{button_class} {personal_class}",
                onclick: move |_e| {
                    filter_.set(ActivityFilter::Personal);
                    offset.set(0);
                },
                UserIcon {
                    class: "w-5 h-5 my-auto",
                }
                "Personal"
            }
            button {
                class: "{button_class} {global_class}",
                onclick: move |_| {
                    filter.set(ActivityFilter::Global);
                    offset.set(0);
                },
                GlobeIcon {
                    class: "w-5 h-5 my-auto",
                }
                "Global"
            }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct ActivityTableProps<'a> {
    pub offset: &'a UseState<u64>,
    pub transfers: Vec<Transfer>,
    pub has_more: bool,
}

#[component]
pub fn ActivityTable<'a>(cx: Scope<'a, ActivityTableProps<'a>>) -> Element {
    let offset = cx.props.offset;
    let transfers = cx.props.transfers.clone();
    let has_more = cx.props.has_more;
    if transfers.is_empty() {
        render! {
            p {
                class: "text-sm text-gray-300 py-2 px-1",
                "No transactions found"
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-col gap-4",
                div {
                    class: "h-full w-full max-w-full overflow-x-scroll",
                    table {
                        class: "h-full w-full",
                        ActivityTableHeader {}
                        for transfer in transfers {
                            render! {
                                ActivityRow {
                                    transfer: transfer
                                }
                            }
                        }
                    }
                }
                ActivityTablePagination {
                    offset: offset,
                    has_more: has_more
                }
            }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct ActivityTablePaginationProps<'a> {
    pub offset: &'a UseState<u64>,
    pub has_more: bool,
}

#[component]
pub fn ActivityTablePagination<'a>(cx: Scope<'a, ActivityTablePaginationProps<'a>>) -> Element {
    render! {
        div {
            class: "flex flex-row justify-between",
            if cx.props.offset.get().gt(&0) {
                render! {
                    button {
                        onclick: move |_| {
                            cx.props.offset.set(cx.props.offset.current().saturating_sub(ACTIVITY_TABLE_PAGE_LIMIT as u64));
                        },
                        class: "rounded-full h-10 w-10 font-semibold hover-100 active-200 transition-colors",
                        "←"
                    }
                }
            } else {
                render! {
                    div{}
                }
            }
            if cx.props.has_more {
                render! {
                    button {
                        onclick: move |_| {
                            cx.props.offset.set(cx.props.offset.current().saturating_add(ACTIVITY_TABLE_PAGE_LIMIT as u64));
                        },
                        class: "rounded-full h-10 w-10 font-semibold hover-100 active-200 transition-colors",
                        "→"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ActivityTableHeader(cx: Scope) -> Element {
    render! {
        thead {
            class: "rounded transition-colors text-xs font-medium text-gray-300",
            th {
                class: "text-left py-2",
                "Action"
            }
            th {
                class: "text-left",
                "Amount"
            }
            th {
                class: "text-left",
                "Memo"
            }
            th {
                class: "text-right",
                "Time"
            }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct ActivityRowProps {
    pub transfer: Transfer,
}

#[component]
pub fn ActivityRow(cx: Scope<ActivityRowProps>) -> Element {
    let transfer = cx.props.transfer.clone();
    let amount = (transfer.amount as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
    let memo = transfer.memo.unwrap_or("–".into());

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = Duration::from_secs(transfer.ts as u64);
    let time = now.saturating_sub(ts);
    let t = time.as_secs();
    const ONE_MIN: u64 = 60;
    const ONE_HOUR: u64 = ONE_MIN * 60;
    const ONE_DAY: u64 = ONE_HOUR * 24;
    let time_str = if t.gt(&ONE_DAY) {
        format!("{}d ago", t.saturating_div(ONE_DAY))
    } else if t.gt(&ONE_HOUR) {
        format!("{}h ago", t.saturating_div(ONE_HOUR))
    } else if t.gt(&ONE_MIN) {
        format!("{}m ago", t.saturating_div(ONE_MIN))
    } else {
        format!("{}s ago", t)
    };

    let address = match transfer.transfer_type {
        TransferType::Claim | TransferType::Mine => transfer.to_address[..5].to_string(),
        TransferType::Spl => transfer.from_address[..5].to_string(),
    };

    render! {
        tr {
        // Link {
            // to: Route::Tx {
            //     sig: transfer.sig
            // },
            class: "rounded hover-100 active-200 transition-colors",
            td {
                class: "text-left py-2 font-mono min-w-32 font-medium text-nowrap",
                span {
                    class: "flex flex-row gap-2",
                    match transfer.transfer_type {
                        TransferType::Claim => {
                            render! {
                                CircleStackIcon {
                                    class: "w-4 h-4 my-auto"
                                }
                            }
                        }
                        TransferType::Mine => {
                            render! {
                                CubeIcon {
                                    class: "w-4 h-4 my-auto"
                                }
                            }
                        }
                        TransferType::Spl => {
                            render! {
                                PaperAirplaneIcon {
                                    class: "w-4 h-4 my-auto"
                                }
                            }
                        }
                    }
                    "{address}"
                }
            }
            td {
                class: "text-left font-medium min-w-40 text-nowrap",
                span {
                    class: "flex flex-row gap-1",
                    OreIcon {
                        class: "w-3.5 h-3.5 my-auto"
                    }
                    "{amount}"
                }
            }
            td {
                class: "text-left text-nowrap min-w-32",
                "{memo}"
            }
            td {
                class: "text-right text-nowrap min-w-16",
                "{time_str}"
            }
        }
    }
}
