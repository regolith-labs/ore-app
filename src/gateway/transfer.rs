use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Signature, signer::Signer};

use super::{get_keypair, ore_token_account_address, send_and_confirm, wasm_client};

// TODO Create destination account if needed

pub async fn transfer(amount: u64, to: Pubkey, memo: String) -> Option<Signature> {
    let client = wasm_client();
    let keypair = get_keypair();
    let from_token_account = ore_token_account_address(keypair.pubkey());
    let to_token_account = ore_token_account_address(to);
    let memo_ix =
        solana_extra_wasm::program::spl_memo::build_memo(&memo.into_bytes(), &[&keypair.pubkey()]);
    let transfer_ix = solana_extra_wasm::program::spl_token::instruction::transfer(
        &solana_extra_wasm::program::spl_token::ID,
        &from_token_account,
        &to_token_account,
        &keypair.pubkey(),
        &[&keypair.pubkey()],
        amount,
    )
    .unwrap();
    send_and_confirm(&client, &[memo_ix, transfer_ix]).await
}
