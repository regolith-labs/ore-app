use dioxus::prelude::*;

use crate::components::{Col, Row};

// Help tab types
#[derive(Clone, Copy, PartialEq)]
pub enum HelpTab {
    Overview,
    Trading,
    Mining,
    Staking,
    Settings,
}

impl HelpTab {
    fn to_string(&self) -> &'static str {
        match self {
            HelpTab::Overview => "Overview",
            HelpTab::Trading => "Trading",
            HelpTab::Mining => "Mining",
            HelpTab::Staking => "Staking",
            HelpTab::Settings => "Settings",
        }
    }
}

#[component]
pub fn HelpDrawer(on_close: EventHandler<MouseEvent>) -> Element {
    let active_tab = use_signal(|| HelpTab::Overview);

    rsx! {
        div {
            class: "flex flex-col h-full w-screen sm:w-96 elevated elevated-border text-white z-50",

            // Header section
            div {
                class: "px-4 pt-4 pb-2",

                // Close button
                button {
                    class: "rounded-full text-center py-1 w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer self-center mb-4",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_close.call(e);
                    },
                    span {
                        class: "text-xl font-semibold",
                        "×"
                    }
                }

                // Title
                div {
                    class: "flex flex-col items-center mb-4",
                    h2 {
                        class: "text-xl font-semibold text-elements-highEmphasis",
                        "Help Center"
                    }
                    p {
                        class: "text-sm text-elements-lowEmphasis",
                        "Learn how to use Ore App"
                    }
                }
            }

            // Tabs navigation
            div {
                class: "px-4 pb-2 border-b border-gray-800",
                div {
                    class: "flex justify-between overflow-x-auto scrollbar-hide",
                    HelpTabButton {
                        tab: HelpTab::Overview,
                        active_tab: active_tab
                    }
                    HelpTabButton {
                        tab: HelpTab::Trading,
                        active_tab: active_tab
                    }
                    HelpTabButton {
                        tab: HelpTab::Mining,
                        active_tab: active_tab
                    }
                    HelpTabButton {
                        tab: HelpTab::Staking,
                        active_tab: active_tab
                    }
                    HelpTabButton {
                        tab: HelpTab::Settings,
                        active_tab: active_tab
                    }
                }
            }

            // Scrollable content area
            div {
                class: "flex-1 overflow-y-auto",
                style: "padding-bottom: 1rem;",
                match *active_tab.read() {
                    HelpTab::Overview => rsx! { HelpContent { title: "Overview" } },
                    HelpTab::Trading => rsx! { HelpContent { title: "Trading" } },
                    HelpTab::Mining => rsx! { HelpContent { title: "Mining" } },
                    HelpTab::Staking => rsx! { HelpContent { title: "Staking" } },
                    HelpTab::Settings => rsx! { HelpContent { title: "Settings" } },
                }
            }
        }
    }
}

#[component]
fn HelpTabButton(tab: HelpTab, active_tab: Signal<HelpTab>) -> Element {
    let is_active = *active_tab.read() == tab;
    let style_class = if is_active {
        "text-elements-highEmphasis border-b-2 border-elements-gold"
    } else {
        "text-elements-lowEmphasis hover:text-elements-midEmphasis"
    };

    rsx! {
        button {
            class: "px-3 py-2 text-sm font-medium whitespace-nowrap transition-colors duration-200 {style_class}",
            onclick: move |_| active_tab.set(tab),
            "{tab.to_string()}"
        }
    }
}

#[component]
fn HelpContent(title: &'static str) -> Element {
    // Dummy content for each tab
    rsx! {
        div {
            class: "px-4 py-4",
            h3 {
                class: "text-lg font-semibold text-elements-highEmphasis mb-2",
                "{title} Help"
            }
            p {
                class: "text-elements-midEmphasis mb-4",
                "This is the help content for the {title} section. Here you can learn how to use this feature of the Ore App."
            }

            // Generate some dummy sections
            for i in 1..5 {
                div {
                    class: "mb-6",
                    h4 {
                        class: "text-md font-semibold text-elements-highEmphasis mb-2",
                        "{title} Topic {i}"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-2",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                    }
                    p {
                        class: "text-elements-midEmphasis mb-2",
                        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                    }
                    ul {
                        class: "list-disc pl-5 text-elements-midEmphasis",
                        li { "Step 1: Do this first thing" }
                        li { "Step 2: Then do this second thing" }
                        li { "Step 3: Finally, complete the process" }
                    }
                }
            }
        }
    }
}
