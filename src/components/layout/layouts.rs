use dioxus::prelude::*;

use crate::components::*;
use crate::hooks::use_sol_balance_wss;
use crate::route::Route;

use super::navigation::*;

pub fn AppLayout() -> Element {
    let sol_balance = use_sol_balance_wss();
    rsx! {
        Col {
            class: "w-screen min-h-dvh",
            AppNavBar { tabs: true }
            div {
                match sol_balance.cloned() {
                    Ok(_) => "ok",
                    Err(_err) => "err",
                }
            }
            MobileTabBar {}
            span {
                class: "py-5 sm:py-8 w-full max-w-[80rem] mx-auto",
                Outlet::<Route> {}
            }
            ToastDisplay {}
        }
    }
}

pub fn AppModalLayout() -> Element {
    rsx! {
        Col {
            class: "w-screen h-screen",
            AppNavBar { tabs: false }
            Outlet::<Route> {}
        }
    }
}

pub fn LandingLayout() -> Element {
    rsx! {
        Col {
            class: "w-screen",
            Outlet::<Route> {}
        }
    }
}

pub fn VisitorLayout() -> Element {
    rsx! {
        Col {
            class: "w-screen h-screen",
            VisitorNavBar {}
            Outlet::<Route> {}
        }
    }
}
