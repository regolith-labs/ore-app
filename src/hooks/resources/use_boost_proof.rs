use std::collections::HashMap;

use dioxus::prelude::*;
use ore_api::{
    consts::MINT_ADDRESS,
    state::{proof_pda, Proof},
};
use ore_boost_api::state::boost_pda;
use steel::Pubkey;

use crate::{
    config::LISTED_BOOSTS,
    gateway::{ore::OreGateway, GatewayError, GatewayResult},
    hooks::use_gateway,
};

pub(crate) fn use_boost_proofs_provider() {
    // Hashmap to cache resources
    let mut boost_proofs = HashMap::new();

    // Idle ORE boost
    let boost_address = boost_pda(MINT_ADDRESS).0;
    let proof_address = proof_pda(boost_address).0;
    boost_proofs.insert(proof_address, use_proof_resource(proof_address));

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        let boost_address = boost_pda(boost_meta.lp_mint).0;
        let proof_address = proof_pda(boost_address).0;
        boost_proofs.insert(proof_address, use_proof_resource(proof_address));
    }

    // Setup context provider
    use_context_provider(|| boost_proofs);
}

fn use_proof_resource(address: Pubkey) -> Resource<GatewayResult<Proof>> {
    use_resource(move || async move {
        use_gateway()
            .get_proof(address)
            .await
            .map_err(GatewayError::from)
    })
}

pub fn use_all_boost_proofs() -> HashMap<Pubkey, Resource<GatewayResult<Proof>>> {
    use_context()
}

pub fn use_boost_proof(mint_address: Pubkey) -> Resource<GatewayResult<Proof>> {
    let boost_proofs: HashMap<Pubkey, Resource<GatewayResult<Proof>>> = use_context();
    let boost_address = boost_pda(mint_address).0;
    let boost_proof_address = proof_pda(boost_address).0;
    if let Some(proof) = boost_proofs.get(&boost_proof_address) {
        *proof
    } else {
        use_proof_resource(boost_proof_address)
    }
}
