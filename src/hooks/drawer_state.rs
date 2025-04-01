use dioxus::prelude::*;

use crate::components::DocsTab;

// Signal for managing the visibility of the wallet drawer
pub type IsWalletDrawerOpen = bool;

// Provider hook for wallet drawer visibility
pub fn use_wallet_drawer_state_provider() {
    use_context_provider::<Signal<IsWalletDrawerOpen>>(|| Signal::new(false));
}

// Hook to get or set the wallet drawer state
pub fn use_wallet_drawer_state() -> Signal<IsWalletDrawerOpen> {
    use_context::<Signal<IsWalletDrawerOpen>>()
}

// Signal for managing the visibility and content of the docs drawer
#[derive(Clone)]
pub struct DocsDrawerState {
    pub is_open: bool,
    pub tab: DocsTab,
}

// Provider hook for help drawer state
pub fn use_docs_drawer_state_provider() {
    use_context_provider::<Signal<DocsDrawerState>>(|| {
        Signal::new(DocsDrawerState {
            is_open: false,
            tab: DocsTab::Mining,
        })
    });
}

// Hook to get or set the help drawer state
pub fn use_docs_drawer_state() -> Signal<DocsDrawerState> {
    use_context::<Signal<DocsDrawerState>>()
}
