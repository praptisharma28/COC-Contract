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

    pub fn assign_user_to_role(ctx: Context<AssignUserToRole>, user: Pubkey) -> Result<()> {
        ctx.accounts.assign_user_to_role(user)
    }

    pub fn onboard_industry(
        ctx: Context<OnboardIndustry>,
        company_name: String,
        registration_number: String,
        bond_amount: u64,
    ) -> Result<()> {
        // derive the bump for the PDA
        let bump = ctx.bumps.industry;
        ctx.accounts
            .onboard_industry(company_name, registration_number, bond_amount, bump)
    }

    pub fn report_emissions(
        ctx: Context<ReportEmission>,
        co2_tonnes: u64,
        reporting_period: String,
    ) -> Result<()> {
        ctx.accounts.report_emissions(co2_tonnes, reporting_period)
    }
}
