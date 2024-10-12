use dioxus::prelude::*;

use crate::{
    components::CheckCircleIcon,
    hooks::use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
    route::Route,
};

use super::Stake;

#[component]
pub fn StakeDone(stake: Stake) -> Element {
    // idempotent register in pool
    let wallet_adapter = use_wallet_adapter();
    let pool = crate::pool::Pool::new(reqwest::Client::new());
    spawn(async move {
        if let WalletAdapter::Connected(pubkey) = *wallet_adapter.read() {
            if let Err(err) = pool.post_register(pubkey).await {
                log::error!("{:?}", err);
            }
            if let Err(err) = pool.post_register_staker(pubkey, stake.mint).await {
                log::error!("{:?}", err);
            }
        }
    });
    let action = format!("You have successfully stake your {}.", stake.name);
    rsx! {
        div {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-col gap-3",
                h2 {
                    "Success!"
                }
                p {
                    class: "text-lg",
                    "{action}"
                }
                p {
                    // TODO: "it can take a few minutes for your stake to register (with the pool)."
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "This will give an extra multiplier on your mining rewards."
                }
            }
            div {
                class: "flex flex-col gap-8 w-full",
                CheckCircleIcon { class: "h-12 w-12 mx-auto" }
                // Link {
                //     class: "font-mono text-nowrap truncate mx-auto w-full p-2 rounded hover-100 active-200",
                //     to: Route::Tx {
                //         sig: signature.to_string(),
                //     },
                //     "{signature.to_string()}"
                // }
            }
            div {
                class: "flex flex-col gap-3",
                div {
                    class: "h-full"
                }
                Link {
                    class: "w-full py-3 rounded font-semibold transition-colors text-center text-white bg-green-500 hover:bg-green-600 active:bg-green-700",
                    to: Route::Home{},
                    "Done"
                }
            }
        }
    }
}
