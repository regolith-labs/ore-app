use jupiter_dca_sdk::{
    accounts::Dca,
    instructions::{OpenDcaV2, OpenDcaV2InstructionArgs},
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::{solana::SolanaGateway, spl::SplGateway, Gateway, GatewayResult, Rpc};
use crate::{
    config::Token,
    solana::{
        spl_associated_token_account::{
            create_associated_token_account_idempotent, get_associated_token_address,
        },
        spl_token,
    },
};
pub trait JupiterGateway {
    async fn build_jupiter_dca_instruction(
        &self,
        user: Pubkey,
        sell_token: Token,
        buy_token: Token,
        sell_amount: u64,
        sell_amount_per_cycle: u64,
        cycle_frequency: i64,
        start_at: Option<i64>,
    ) -> GatewayResult<Vec<Instruction>>;
}

impl<R: Rpc + SplGateway + SolanaGateway> JupiterGateway for Gateway<R> {
    async fn build_jupiter_dca_instruction(
        &self,
        user: Pubkey,
        sell_token: Token,
        buy_token: Token,
        sell_amount: u64,
        sell_amount_per_cycle: u64,
        cycle_frequency: i64,
        start_at: Option<i64>,
    ) -> GatewayResult<Vec<Instruction>> {
        // Get current timestamp
        let clock = self.rpc.get_clock().await?;
        let current_time = clock.unix_timestamp;
        let timestamp = current_time.to_le_bytes();

        // Jup DCA Program ID
        let dca_program_id =
            Pubkey::from_str("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M").unwrap();

        // Derive DCA PDA for current user
        let dca_pda = Pubkey::find_program_address(
            &[b"dca", sell_token.as_ref(), buy_token.as_ref(), &timestamp],
            &dca_program_id,
        )
        .0;

        // Get ATAs
        let user_ata = get_associated_token_address(&user, &sell_token);
        let in_ata = get_associated_token_address(&dca_pda, &sell_token);
        let out_ata = get_associated_token_address(&dca_pda, &buy_token);

        // Instructions
        let mut ixs = vec![];

        // Create ATAs for PDA
        let create_in_ata_ix = create_associated_token_account_idempotent(
            &user,
            &dca_pda,
            &sell_token.mint,
            &spl_token::ID,
        );
        ixs.push(create_in_ata_ix);

        // Create ATAs for PDA
        let create_out_ata_ix = create_associated_token_account_idempotent(
            &user,
            &dca_pda,
            &buy_token.mint,
            &spl_token::ID,
        );
        ixs.push(create_out_ata_ix);

        // Build args
        let args = OpenDcaV2InstructionArgs {
            application_idx: current_time as u64,
            in_amount: sell_amount,
            in_amount_per_cycle: sell_amount_per_cycle,
            cycle_frequency: cycle_frequency,
            min_out_amount: None,
            max_out_amount: None,
            start_at: start_at.unwrap_or(current_time as u64),
        };

        let accounts = OpenDcaV2 {
            dca: dca_pda,
            user,
            payer: user,
            input_mint: sell_token,
            output_mint: buy_token,
            user_ata,
            in_ata,
            out_ata,
            system_program: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            token_program: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            associated_token_program: Pubkey::from_str(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            )
            .unwrap(),
            event_authority: Pubkey::from_str("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj")
                .unwrap(),
            program: dca_program_id,
        };
        ixs.push(accounts.instruction(args));

        Ok(ixs)
    }
}
