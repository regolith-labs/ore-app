use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

use super::navigation::*;

pub fn AppLayout() -> Element {
    rsx! {
        Col {
            class: "w-screen min-h-dvh",
            AppNavBar {}
            MobileTabBar {}
            span {
                class: "py-5 sm:py-8 w-full max-w-[80rem] mx-auto",
                Outlet::<Route> {}
            }
            ToastDisplay {}
        }
    }
}

pub fn LandingLayout() -> Element {
    rsx! {
        Col {
            class: "w-screen",
            // LandingNavBar {}
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
