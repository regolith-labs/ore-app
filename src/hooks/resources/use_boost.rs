use std::collections::HashMap;

use dioxus::prelude::*;
use ore_api::{
    consts::MINT_ADDRESS,
    state::{proof_pda, Proof},
};
use ore_boost_api::state::{boost_pda, Boost};
use steel::Pubkey;

use crate::{
    config::LISTED_BOOSTS,
    gateway::{ore::OreGateway, GatewayError, GatewayResult},
    hooks::use_gateway,
};

// TODO: REFerence resoourcee
// we can call restart on the resource
pub(crate) fn use_boosts_provider() {
    // Hashmap to cache resources
    let mut boosts = HashMap::new();

    // Idle ORE boost
    let boost_address = boost_pda(MINT_ADDRESS).0;
    boosts.insert(boost_address, use_boost_resource(boost_address));

    // TODO: for the listed, go thrugh the list of tokens and fetch the atas ffr tthe current wallet
    // in the cases waallet changes, then we would reset the reseurrce and refetch the new atas

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        let boost_address = boost_pda(boost_meta.lp_mint).0;
        boosts.insert(boost_address, use_boost_resource(boost_address));
    }

    // Setup context provider
    use_context_provider(|| boosts);
}

fn use_boost_resource(address: Pubkey) -> Resource<GatewayResult<Boost>> {
    use_resource(move || async move {
        use_gateway()
            .get_boost(address)
            .await
            .map_err(GatewayError::from)
    })
}

/*
use the tokens resurce
fetch the


esseentilly the use_bosot but use token accounts instead
*/
pub fn use_boost(mint_address: Pubkey) -> Resource<GatewayResult<Boost>> {
    let boosts: HashMap<Pubkey, Resource<GatewayResult<Boost>>> = use_context();
    let boost_address = boost_pda(mint_address).0;
    if let Some(boost) = boosts.get(&boost_address) {
        *boost
    } else {
        use_boost_resource(boost_address)
    }
}

pub fn use_boost_proof(mint_address: Pubkey) -> Resource<GatewayResult<Proof>> {
    let boost_address = boost_pda(mint_address).0;
    let boost_proof_address = proof_pda(boost_address).0;
    use_resource(move || async move {
        use_gateway()
            .get_proof(boost_proof_address)
            .await
            .map_err(GatewayError::from)
    })

    // let boosts: HashMap<Pubkey, Resource<GatewayResult<Boost>>> = use_context();
    // let boost_address = boost_pda(mint_address).0;
    // if let Some(boost) = boosts.get(&boost_address) {
    //     *boost
    // }
}
