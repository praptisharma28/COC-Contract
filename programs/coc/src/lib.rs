use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod states;
pub mod utils;

declare_id!("67NqH9UEtNMZCcqs1iAZG5TWJzfyqK8DgYnvGx1EECBL");

#[program]
pub mod coc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
