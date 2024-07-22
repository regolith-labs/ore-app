use dioxus::prelude::*;
use ore_types::{response::ListTransfersResponse, Transfer, TransferType};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{GlobeIcon, OreIcon, UserBubble, UserIcon},
    hooks::{use_transfers, ActivityFilter, ACTIVITY_TABLE_PAGE_LIMIT},
    route::Route,
};

use super::wallet_adapter;

pub fn Activity() -> Element {
    let filter = use_signal(|| ActivityFilter::Global);
    let offset = use_signal(|| 0u64);
    let mut transfers = use_transfers(filter, offset);

    use_future(move || async move {
        loop {
            async_std::task::sleep(Duration::from_secs(30)).await;
            transfers.restart();
        }
    });

    let e = if let Some(transfers) = transfers.read().clone() {
        match transfers {
            Ok(transfers) => {
                rsx! {
                    div {
                        class: "flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start",
                        div {
                            class: "flex flex-row justify-between",
                            h2 {
                                class: "text-lg md:text-2xl font-bold my-auto",
                                "Activity"
                            }
                            // FilterButtons {
                            //     filter,
                            //     offset
                            // }
                        }
                        ActivityTable {
                            offset,
                            transfers
                        }
                    }
                }
            }
            _ => rsx! {},
        }
    } else {
        rsx! {
            div {
                class: "flex flex-row h-64 w-full loading rounded",
            }
        }
    };
    e
}

#[component]
pub fn FilterButtons(filter: Signal<ActivityFilter>, offset: Signal<u64>) -> Element {
    let selected_class = "";
    let unselected_class = "text-gray-300 dark:text-gray-700";
    let (global_class, personal_class) = match *filter.read() {
        ActivityFilter::Global => (selected_class, unselected_class),
        ActivityFilter::Personal => (unselected_class, selected_class),
    };
    let button_class =
        "flex flex-row gap-2 px-2 md:px-3 py-2 rounded-full text-xs md:text-sm hover-100 active-200 transition-colors";
    let icon_class = "w-4 h-4 md:w-5 md:h-5 my-auto";

    rsx! {
        div {
            class: "flex flex-row gap-1 md:gap-2 font-semibold -mx-1 md:-mx-2",
            button {
                class: "{button_class} {personal_class}",
                onclick: move |_e| {
                    filter.set(ActivityFilter::Personal);
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

#[component]
pub fn ActivityTable(offset: Signal<u64>, transfers: ListTransfersResponse) -> Element {
    if transfers.data.is_empty() {
        rsx! {
            p {
                class: "text-sm text-gray-300 py-2 sm:px-1",
                "No transactions found"
            }
        }
    } else {
        rsx! {
            div {
                class: "flex flex-col gap-4 -mx-2 sm:mx-0",
                div {
                    class: "h-full w-full max-w-full",
                    for transfer in transfers.data {
                        ActivityRow {
                            transfer
                        }
                    }
                }
                ActivityTablePagination {
                    offset,
                    has_more: transfers.has_more
                }
            }
        }
    }
}

#[component]
pub fn ActivityTablePagination(offset: Signal<u64>, has_more: bool) -> Element {
    let should_show = offset.read().gt(&0);
    let mut offset = offset.clone();
    rsx! {
        div {
            class: "flex flex-row justify-between",
            if should_show {
                button {
                    onclick: move |_| {
                        let page_down = offset.read().saturating_sub(ACTIVITY_TABLE_PAGE_LIMIT as u64);
                        offset.set(page_down);
                    },
                    class: "rounded-full h-10 w-10 font-semibold hover-100 active-200 transition-colors",
                    "←"
                }
            } else {
                div {}
            }
            if has_more {
                button {
                    onclick: move |_| {
                        let page_up = offset.read().saturating_add(ACTIVITY_TABLE_PAGE_LIMIT as u64);
                        offset.set(page_up);
                    },
                    class: "rounded-full h-10 w-10 font-semibold hover-100 active-200 transition-colors",
                    "→"
                }
            }
        }
    }
}

#[component]
pub fn ActivityRow(transfer: Transfer) -> Element {
    // TODO let pubkey = use_pubkey();
    let pubkey = Pubkey::new_from_array([0; 32]);
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
    let addr_a_link = match transfer.transfer_type {
        TransferType::Claim | TransferType::Mine => transfer.to_address.clone(),
        TransferType::Spl => transfer.from_address,
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
    let addr_b_link = match transfer.transfer_type {
        TransferType::Claim | TransferType::Mine => "".to_string(),
        TransferType::Spl => transfer.to_address,
    };
    let addr_b_class = if addr_b.eq(&"You".to_string()) {
        "font-bold"
    } else {
        "font-mono font-bold"
    };

    rsx! {
        Link {
            class: "flex flex-row py-3 gap-3 w-full px-2 rounded hover-100 active-200 transition-colors",
            to: Route::Tx { sig: transfer.sig },
            Link {
                class: "hover:opacity-80 transition-opacity",
                to: Route::User { id: addr_a_link.clone() },
                UserBubble {
                    class: "w-10 h-10"
                }
            }
            div {
                class: "flex flex-col gap-2",
                div {
                    class: "flex flex-col gap-0.5 pt-1.5",
                    p {
                        class: "flex flex-row gap-1.5 text-wrap flex-wrap",
                        Link {
                            to: Route::User { id: addr_a_link },
                            span {
                                class: "{addr_a_class} hover:underline",
                                "{addr_a}"
                            }
                        }
                        "{action} "
                        span {
                            class: "flex flex-row font-semibold gap-[0.16rem]",
                            OreIcon {
                                class: "ml-0.5 w-3.5 h-3.5 my-auto",
                            }
                            "{amount_to_ui_amount(transfer.amount as u64, ore_api::consts::TOKEN_DECIMALS)}"
                        }
                        if let TransferType::Spl = transfer.transfer_type {
                            "to"
                            Link {
                                to: Route::User{ id: addr_b_link },
                                span {
                                    class: "{addr_b_class} hover:underline",
                                    "{addr_b}"
                                }
                            }
                        }
                    }
                    p {
                        class: "text-gray-300 text-nowrap text-sm",
                        "{time_str}"
                    }
                }
                if let Some(memo) = transfer.memo {
                    p {
                        "{memo}"
                    }
                }
            }
        }
    }
}
