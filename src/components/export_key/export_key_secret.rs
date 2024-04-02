use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{
    components::{Copyable, EyeSlashIcon},
    hooks::use_keypair,
    route::Route,
};

#[component]
pub fn ExportKeySecret(cx: Scope) -> Element {
    let keypair = use_keypair(cx);
    render! {
        div {
            class: "flex flex-col gap-16 grow w-full h-full",
            div {
                class: "flex flex-col gap-3 justify-start",
                h2 {
                    class: "text-nowrap",
                    "Private key"
                }
                p {
                    class: "text-lg",
                    "Never give out your private key."
                }
            }
            EyeSlashIcon {
                class: "w-12 h-12 mx-auto opacity-50"
            }
            Copyable {
                class: "mx-auto w-full",
                value: keypair.to_base58_string(),
                p {
                    class: "p-2 text-wrap break-words",
                    "{keypair.to_base58_string()}"
                }
            }
            Link {
                to: Route::Settings {},
                class: "bg-green-500 hover:bg-green-600 active:bg-green-700 transition-colors text-white rounded text-center font-semibold py-3 mt-auto",
                "Done"
            }
        }
    }
}
