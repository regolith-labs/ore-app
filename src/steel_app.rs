pub mod solana {
    #[cfg(feature = "desktop")]
    pub mod sdk {
        pub use solana_sdk::*;
    }

    #[cfg(feature = "web")]
    pub mod sdk {
        pub use solana_client_wasm::solana_sdk::*;
    }

    #[cfg(feature = "web")]
    pub mod program {
        pub use solana_extra_wasm::program::*;
    }

    pub mod account_decoder {
        #[cfg(feature = "web")]
        pub use solana_extra_wasm::account_decoder::*;

        #[cfg(feature = "desktop")]
        pub use solana_account_decoder::*;
    }

    pub mod transaction_status {
        #[cfg(feature = "web")]
        pub use solana_extra_wasm::transaction_status::*;

        #[cfg(feature = "desktop")]
        pub use solana_transaction_status::*;
    }
}

pub mod time {
    #[cfg(feature = "desktop")]
    pub use std::time::*;

    #[cfg(feature = "web")]
    pub use web_time::*;
}
