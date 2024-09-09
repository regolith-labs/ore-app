use dioxus::prelude::*;
use ore_api::state::Proof;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::{escrow_pubkey, GatewayError, GatewayResult};

use super::{
    use_escrow, use_gateway,
    use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

pub fn use_proof() -> Resource<GatewayResult<Proof>> {
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletAdapter::Connected(pubkey) => {
                let gateway = use_gateway();
                gateway.get_proof(pubkey).await
            }
        }
    })
}

pub fn use_escrow_proof() -> Resource<GatewayResult<Proof>> {
    let escrow = use_escrow();
    use_resource(move || async move {
        if let Some(Ok(escrow)) = *escrow.read() {
            let authority = escrow.authority;
            if authority.ne(&Pubkey::new_from_array([0; 32])) {
                let gateway = use_gateway();
                gateway.get_proof(escrow_pubkey(authority)).await
            } else {
                Err(GatewayError::AccountNotFound.into())
            }
        } else {
            Err(GatewayError::AccountNotFound.into())
        }
    })
}

pub fn use_user_proof(authority: Pubkey) -> Resource<GatewayResult<Proof>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move { gateway.get_proof(authority).await }
    })
}

pub fn use_proof_v1() -> Resource<GatewayResult<ore_api_v1::state::Proof>> {
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Connected(pubkey) => {
                let gateway = use_gateway();
                gateway.get_proof_v1(pubkey).await
            }
            WalletAdapter::Disconnected => Err(GatewayError::AccountNotFound.into()),
        }
    })
}
