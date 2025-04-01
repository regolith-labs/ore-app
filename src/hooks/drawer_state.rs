use dioxus::prelude::*;

#[derive(Clone)]
pub enum HelpDrawerPage {
    Mine,
    Stake,
}

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

// Signal for managing the visibility and content of the help drawer
#[derive(Clone)]
pub struct HelpDrawerState {
    pub is_open: bool,
}

// Provider hook for help drawer state
pub fn use_help_drawer_state_provider() {
    use_context_provider::<Signal<HelpDrawerState>>(|| {
        Signal::new(HelpDrawerState { is_open: false })
    });
}

// Hook to get or set the help drawer state
pub fn use_help_drawer_state() -> Signal<HelpDrawerState> {
    use_context::<Signal<HelpDrawerState>>()
}
