use dioxus::prelude::*;

use crate::components::TransactionStatus;

pub fn use_transaction_status_provider() {
    use_context_provider(|| Signal::<Option<TransactionStatus>>::new(None));
}

pub fn use_transaction_status() -> Signal<Option<TransactionStatus>> {
    use_context::<Signal<Option<TransactionStatus>>>()
}
