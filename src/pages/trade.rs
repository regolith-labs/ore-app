use dioxus::prelude::*;

use crate::{
    components::*, gateway::GatewayResult, hooks::{use_token_balance, Asset, ASSETS}, route::Route, steel_app::solana::{account_decoder::parse_token::UiTokenAmount, sdk::pubkey::Pubkey}
};

pub fn Trade() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 4,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Trade",
                subtitle: "Swap tokens at the best available price."
            }
            SwapForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                mint_a: Pubkey::new_unique(),
                mint_b: Pubkey::new_unique(),
            }
        }
    }
}

fn Header() -> Element {
    rsx! {
        Row {
            class: "justify-between h-10 px-5 sm:px-8",
            gap: 4,
            span {
                class: "font-wide text-2xl sm:text-3xl font-semibold align-text-bottom my-auto",
                "Balance"
            }
        }
    }
}

fn SwapButton() -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "h-10 controls-primary rounded-full px-4 gap-2 -mr-2",
            SwapIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Swap"
            }
        }
    }
}

