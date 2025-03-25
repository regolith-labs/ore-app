// #[cfg(not(feature = "web"))]
mod submit_transaction_native;
#[cfg(feature = "web")]
mod submit_transaction_web;
mod transaction_status;

#[cfg(not(feature = "web"))]
pub use submit_transaction_native::*;
#[cfg(feature = "web")]
pub use submit_transaction_web::*;
pub use transaction_status::*;

pub struct SecondSigner {
    pub signer: solana_sdk::signature::null_signer::NullSigner,
    pub payer: bool,
}

pub enum MixedSigners {
    Present(solana_sdk::signer::keypair::Keypair),
    Absent(solana_sdk::signature::null_signer::NullSigner),
}

impl solana_sdk::signer::Signer for MixedSigners {
    fn try_pubkey(&self) -> Result<solana_sdk::pubkey::Pubkey, solana_sdk::signer::SignerError> {
        match self {
            Self::Present(keypair) => keypair.try_pubkey(),
            Self::Absent(null_signer) => null_signer.try_pubkey(),
        }
    }

    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        match self {
            Self::Present(keypair) => keypair.try_sign_message(message),
            Self::Absent(null_signer) => null_signer.try_sign_message(message),
        }
    }

    fn is_interactive(&self) -> bool {
        match self {
            Self::Present(keypair) => keypair.is_interactive(),
            Self::Absent(null_signer) => null_signer.is_interactive(),
        }
    }
}
