use cached::proc_macro::cached;
use ore::{BUS_ADDRESSES, MINT_ADDRESS, PROOF, TREASURY_ADDRESS};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::program::spl_associated_token_account::get_associated_token_address;

#[cached]
pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ore::ID).0
}

#[cached]
pub fn bus_pubkey(id: u8) -> Pubkey {
    BUS_ADDRESSES[id as usize]
}

#[cached]
pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}
