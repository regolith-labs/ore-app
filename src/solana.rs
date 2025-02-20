#[cfg(feature = "web")]
pub mod spl_token {
    pub use solana_extra_wasm::program::spl_token::*;
}
#[cfg(not(feature = "web"))]
pub mod spl_token {
    pub use spl_token::*;
}

#[cfg(feature = "web")]
pub mod spl_associated_token_account {
    pub use solana_extra_wasm::program::spl_associated_token_account::*;
}
#[cfg(not(feature = "web"))]
pub mod spl_associated_token_account {
    pub use spl_associated_token_account::*;
}

#[cfg(feature = "web")]
pub mod spl_memo {
    pub use solana_extra_wasm::program::spl_memo::*;
}
#[cfg(not(feature = "web"))]
pub mod spl_memo {
    pub use spl_memo::*;
}

#[cfg(feature = "web")]
pub mod spl_token_2022 {
    pub use solana_extra_wasm::program::spl_token_2022::*;
}
#[cfg(not(feature = "web"))]
pub mod spl_token_2022 {
    pub use spl_token_2022::*;
}
