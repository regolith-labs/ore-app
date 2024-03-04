use dioxus_router::components::Link;
#[cfg(feature = "desktop")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[cfg(feature = "web")]
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use ore_types::{Transfer, TransferType};

use crate::{
    components::{GlobeIcon, OreIcon, UserIcon},
    gateway::AsyncResult,
    hooks::{use_pubkey, use_transfers, ACTIVITY_TABLE_PAGE_LIMIT},
    route::Route,
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
                            class: "text-lg md:text-2xl font-bold my-auto",
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
        "flex flex-row gap-2 px-2 md:px-3 py-2 rounded-full text-xs md:text-sm hover-100 active-200 transition-colors";
    let icon_class = "w-4 h-4 md:w-5 md:h-5 my-auto";
    render! {
        div {
            class: "flex flex-row gap-1 md:gap-2 font-semibold -mx-1 md:-mx-2",
            button {
                class: "{button_class} {personal_class}",
                onclick: move |_e| {
                    filter_.set(ActivityFilter::Personal);
                    offset.set(0);
                },
                UserIcon {
                    class: "{icon_class}"
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
                    class: "{icon_class}"
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
                class: "text-sm text-gray-300 py-2 sm:px-1",
                "No transactions found"
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-col gap-4 -mx-2 sm:mx-0",
                div {
                    class: "h-full w-full max-w-full",
                    for transfer in transfers {
                        render! {
                            ActivityRow {
                                transfer: transfer
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

#[derive(Props, PartialEq)]
pub struct ActivityRowProps {
    pub transfer: Transfer,
}

#[component]
pub fn ActivityRow(cx: Scope<ActivityRowProps>) -> Element {
    let transfer = cx.props.transfer.clone();
    let amount = (transfer.amount as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
    let pubkey = use_pubkey(cx);

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

    let action = match transfer.transfer_type {
        TransferType::Claim => "claimed",
        TransferType::Mine => "mined",
        TransferType::Spl => "sent",
    };

    let addr_a = match transfer.transfer_type {
        TransferType::Claim | TransferType::Mine => {
            if transfer.to_address.eq(&pubkey.to_string()) {
                "You".to_string()
            } else {
                transfer.to_address[..5].to_string()
            }
        }
        TransferType::Spl => {
            if transfer.from_address.eq(&pubkey.to_string()) {
                "You".to_string()
            } else {
                transfer.from_address[..5].to_string()
            }
        }
    };
    let addr_a_class = if addr_a.eq(&"You".to_string()) {
        "font-bold"
    } else {
        "font-mono font-bold"
    };

    let addr_b = if transfer.to_address.eq(&pubkey.to_string()) {
        "You".to_string()
    } else {
        match transfer.transfer_type {
            TransferType::Claim | TransferType::Mine => "".to_string(),
            TransferType::Spl => transfer.to_address[..5].to_string(),
        }
    };
    let addr_b_class = if addr_b.eq(&"You".to_string()) {
        "font-bold"
    } else {
        "font-mono font-bold"
    };

    render! {
        Link {
            class: "flex flex-row py-3 gap-3 w-full px-2 rounded hover-100 active-200 transition-colors",
            to: Route::Tx { sig: transfer.sig },
            div {
                class: "w-10 h-10 bg-gray-300 dark:bg-gray-700 rounded-full"
            }
            div {
                class: "flex flex-col gap-2",
                div {
                    class: "flex flex-col gap-0.5 pt-1.5",
                    p {
                        class: "flex flex-row gap-1.5",
                        span {
                            class: "{addr_a_class}",
                            "{addr_a}"
                        }
                        "{action} "
                        span {
                            class: "flex flex-row font-semibold gap-[0.16rem]",
                            OreIcon {
                                class: "ml-0.5 w-3.5 h-3.5 my-auto",
                            }
                            "{amount}"
                        }
                        if let TransferType::Spl = transfer.transfer_type {
                            render! {
                                "to"
                                span {
                                    class: "{addr_b_class}",
                                    "{addr_b}"
                                }
                            }
                        }
                    }
                    p {
                        class: "opacity-50 text-nowrap text-sm",
                        "{time_str}"
                    }
                }
                if let Some(memo) = transfer.memo {
                    render! {
                        p {
                            "{memo}"
                        }
                    }
                }
            }
        }
    }
}
