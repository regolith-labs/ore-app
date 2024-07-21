use dioxus::prelude::*;
use ore_relayer_api::state::Escrow;

pub fn use_escrow() -> Signal<Escrow> {
    use_context::<Signal<Escrow>>()
}

pub fn use_escrow_provider() {
    use_context_provider(|| Signal::new(Escrow::default()));
}
