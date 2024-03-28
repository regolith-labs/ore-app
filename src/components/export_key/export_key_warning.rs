use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{
    components::{ChatButtonRightLeftIcon, FlagIcon, LockOpenIcon},
    route::Route,
};

#[component]
pub fn ExportKeyWarning(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-16 w-full h-full",
            div {
                class: "flex flex-col gap-3 justify-start",
                h2 {
                    class: "text-nowrap",
                    "Export key"
                }
            }
            div {
                class: "flex flex-row gap-4",
                ChatButtonRightLeftIcon {
                    class: "w-6 h-6"
                },
                p {
                    class: "text-lg",
                    "Ore will never ask for your private key."
                }
            }
            div {
                class: "flex flex-row gap-4",
                FlagIcon {
                    class: "w-6 h-6"
                }
                p {
                    class: "text-lg",
                    "Never share your private key or enter it into an app or website."
                }
            }
            div {
                class: "flex flex-row gap-4",
                LockOpenIcon {
                    class: "w-6 h-6"
                }
                p {
                    class: "text-lg",
                    "Anyone with your private key will have complete control of your account."
                }
            }
            Link {
                to: Route::ExportKeySecret {},
                class: "bg-green-500 hover:bg-green-600 active:bg-green-700 transition-colors text-white rounded text-center font-semibold py-3 mt-auto",
                "I understand. Continue"
            }
        }
    }
}
