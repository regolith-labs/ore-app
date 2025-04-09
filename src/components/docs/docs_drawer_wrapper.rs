use crate::{
    components::{Col, MiningContent, Row, StakingContent, TokenomicsContent},
    hooks::{use_docs_drawer_state, DocsDrawerState},
};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum DocsTab {
    Mining,
    Staking,
    Tokenomics,
}

#[component]
pub fn DocsDrawerWrapper(
    drawer_state: Signal<DocsDrawerState>,
    on_close: EventHandler<MouseEvent>,
    drawer_remount: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "fixed right-0 top-0 flex flex-col h-full w-screen overflow-y-scroll sm:w-[574px] elevated border-l border-gray-800 text-white z-50 transition-transform duration-300 ease-in-out transform translate-x-0",
            onclick: move |e| e.stop_propagation(),
            DocsContent { on_close: on_close.clone() }
        }
    }
}

#[component]
fn DocsCloseButton(on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "rounded-full text-center py-1 w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer",
            onclick: move |e| {
                e.stop_propagation();
                on_close.call(e);
            },
            span {
                class: "text-xl font-semibold",
                "×"
            }
        }
    }
}
#[component]
fn DocsContent(on_close: EventHandler<MouseEvent>) -> Element {
    let docs_state = use_docs_drawer_state();
    let current_tab = use_memo(move || docs_state.read().tab);
    rsx! {
        Fragment {
            Col {
                class: "w-full py-8" ,
                gap: 8,
                DocsHeader { on_close: on_close.clone() }
                Row {
                    class: "w-full mb-4 bg-surface-elevated border-b border-gray-800",
                    DocsTabButton { tab: DocsTab::Mining }
                    DocsTabButton { tab: DocsTab::Staking }
                    DocsTabButton { tab: DocsTab::Tokenomics }
                }
                div {
                    match *current_tab.read() {
                        DocsTab::Mining => rsx! { MiningContent {} },
                        DocsTab::Staking => rsx! { StakingContent {} },
                        DocsTab::Tokenomics => rsx! { TokenomicsContent {} },
                    }
                }
            }
        }
    }
}

#[component]
fn DocsHeader(on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        Row {
            class: "px-8 justify-between",
            Col {
                gap: 2,
                span {
                    class: "text-3xl font-bold font-wide",
                    "Docs"
                }
                span {
                    class: "text-md text-elements-lowEmphasis font-medium",
                    "Learn more about ORE."
                }
            }
            DocsCloseButton {
                on_close: on_close.clone()
            }
        }

    }
}

#[component]
fn DocsTabButton(tab: DocsTab) -> Element {
    let mut docs_state = use_docs_drawer_state();
    let current_tab = use_memo(move || docs_state.read().tab);
    let title = match tab {
        DocsTab::Mining => "Mining",
        DocsTab::Staking => "Staking",
        DocsTab::Tokenomics => "Tokenomics",
    };
    rsx! {
        button {
            class: "flex-1 h-12 transition-colors font-semibold hover:cursor-pointer border-b",
            class: if *current_tab.read() == tab {
                "text-lg text-white border-controls-primary"
            } else {
                "text-lg text-elements-lowEmphasis"
            },
            onclick: move |_| {
                let mut current = docs_state.read().clone();
                current.tab = tab;
                docs_state.set(current);
            },
            "{title}"
        }
    }
}
