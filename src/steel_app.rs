pub mod solana {
    pub mod client {
        #[cfg(feature = "web")]
        pub use solana_client_wasm::*;

        #[cfg(not(feature = "web"))]
        pub use solana_client::*;
    }

    pub mod program {
        #[cfg(feature = "web")]
        pub use solana_extra_wasm::program::spl_associated_token_account::*;

        #[cfg(not(feature = "web"))]
        pub use spl_associated_token_account::*;
    }

    pub mod account_decoder {
        #[cfg(feature = "web")]
        pub use solana_extra_wasm::account_decoder::*;

        #[cfg(not(feature = "web"))]
        pub use solana_account_decoder::*;
    }

    pub mod transaction_status {
        #[cfg(feature = "web")]
        pub use solana_extra_wasm::transaction_status::*;

        #[cfg(not(feature = "web"))]
        pub use solana_transaction_status::*;
    }
}

pub mod time {
    #[cfg(feature = "web")]
    pub use web_time::*;

    #[cfg(not(feature = "web"))]
    pub use std::time::*;
}
