use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

declare_id!("67NqH9UEtNMZCcqs1iAZG5TWJzfyqK8DgYnvGx1EECBL");

#[program]
pub mod coc {
    use super::*;
    use crate::instructions::*;
    use crate::states::UserRole;

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
        user_roles: Vec<Account<UserRole>>,
    ) -> Result<()> {
        ctx.accounts.assign_user_to_role(user, user_roles)
    }
}
