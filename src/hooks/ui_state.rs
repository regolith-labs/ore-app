use dioxus::prelude::*;

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

// Signal for managing the visibility of the help drawer
pub type IsHelpDrawerOpen = bool;

// Provider hook for help drawer visibility
pub fn use_help_drawer_state_provider() {
    use_context_provider::<Signal<IsHelpDrawerOpen>>(|| Signal::new(false));
}

// Hook to get or set the help drawer state
pub fn use_help_drawer_state() -> Signal<IsHelpDrawerOpen> {
    use_context::<Signal<IsHelpDrawerOpen>>()
}
