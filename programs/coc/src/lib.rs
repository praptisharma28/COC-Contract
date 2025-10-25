use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod states;
mod utils;

pub use instructions::*;
pub use states::*;

declare_id!("4yU1TuZVZ38AXdw5ws1EHW4yqPRFjxb6BjZC2SB5C1gY");

#[program]
pub mod coc {
    use super::*;

    pub fn initialize_access_control(ctx: Context<InitializeAccessControl>) -> Result<()> {
        let bump = ctx.bumps.controller;
        ctx.accounts.initialize_access_control(bump)
    }

    pub fn create_role(
        ctx: Context<CreateRole>,
        role_name: String,
        actions: Vec<String>,
    ) -> Result<()> {
        let bump = ctx.bumps.user_role;
        ctx.accounts.create_role(role_name, actions, bump)
    }

    pub fn assign_user_to_role(
        ctx: Context<AssignUserToRole>,
        user: Pubkey,
    ) -> Result<()> {
        ctx.accounts.assign_user_to_role(user)
    }
}
