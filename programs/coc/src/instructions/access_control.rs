// This file governs who can do what in your system.

use crate::errors::ErrorCode;
use crate::states::{Controller, UserRole};
use crate::utils::has_permission;
use anchor_lang::prelude::*;

/// ------------------------------------------------------------------------
///  INITIALIZE ACCESS CONTROL
/// ------------------------------------------------------------------------
/// This instruction initializes the global Controller PDA.
/// It sets up the default admin who will control roles & permissions.
/// Should be called only once at deployment.
///
#[derive(Accounts)]
pub struct InitializeAccessControl<'info> {
    ///  The signer who initializes the access control system.
    ///    This wallet becomes the `default_admin` for the entire program.
    ///    Must pay for the PDA creation.
    #[account(mut)]
    pub admin: Signer<'info>,

    ///  The PDA storing the controller (root of access control).
    ///    - Contains `default_admin` pubkey.
    ///    - Derived from static seed "controller" (unique to program).
    ///    - Only created once.
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1, // discriminator + fields
        seeds = [b"controller"],
        bump
    )]
    pub controller: Account<'info, Controller>,

    ///  System program for account creation.
    pub system_program: Program<'info, System>,
}

/// ------------------------------------------------------------------------
/// CREATE ROLE
/// ------------------------------------------------------------------------
/// This instruction creates a new UserRole PDA linked to the controller.
/// Only the default admin can call this.
/// Example roles: "Verifier", "Issuer", "IndustryAdmin".
///
#[derive(Accounts)]
#[instruction(role_name: String)]
pub struct CreateRole<'info> {
    ///  The main controller PDA.
    ///    Ensures that only the program’s main authority can create roles.
    #[account(mut, has_one = default_admin)]
    pub controller: Account<'info, Controller>,

    ///  The PDA for the new role being created.
    ///    - Unique seed derived from "role" + role_name
    ///    - Stores allowed actions and assigned users
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 64 + (4 + 10 * 32) + (4 + 10 * 32), // adjust for dynamic vec sizes
        seeds = [b"role", role_name.as_bytes()],
        bump
    )]
    pub user_role: Account<'info, UserRole>,

    ///  The signer creating the role.
    ///    Must match the controller’s `default_admin`.
    #[account(mut)]
    pub admin: Signer<'info>,

    ///  System program for PDA allocation.
    pub system_program: Program<'info, System>,
}

/// ------------------------------------------------------------------------
/// ASSIGN USER TO ROLE
/// ------------------------------------------------------------------------
/// Adds a specific wallet (user) to an existing role.
/// Only the default admin or authorized assigner can call this.
///
#[derive(Accounts)]
pub struct AssignUserToRole<'info> {
    /// The role account being updated.
    ///    - Must belong to a valid controller.
    ///    - Holds list of users & allowed actions.
    #[account(mut, has_one = controller)]
    pub user_role: Account<'info, UserRole>,

    ///  Reference to the main controller PDA.
    ///    - Used for authority validation.
    #[account(mut)]
    pub controller: Account<'info, Controller>,

    ///  The signer assigning a user to the role.
    ///    - Must be the default admin (for now).
    #[account(mut)]
    pub admin: Signer<'info>,
}

impl<'info> InitializeAccessControl<'info> {
    pub fn initialize_access_control(&mut self, bump: u8) -> Result<()> {
        //  Ensure it runs only once
        require!(
            self.controller.to_account_info().data_is_empty(),
            ErrorCode::AlreadyInitialized
        );

        //  Ensure signer == admin
        require!(self.admin.is_signer, ErrorCode::AdminMustSign);

        // Set the default admin to the signer
        self.controller.default_admin = *self.admin.key;
        self.controller.bump = bump;
        Ok(())
    }
}

impl<'info> CreateRole<'info> {
    pub fn create_role(&mut self, role_name: String, actions: Vec<String>, bump: u8) -> Result<()> {
        //  Ensure only default admin can create roles
        require!(
            self.admin.key() == self.controller.default_admin,
            ErrorCode::Unauthorized
        );

        // Ensuring non empty role name
        require!(!role_name.trim().is_empty(), ErrorCode::InvalidRoleName);

        //  Initialize the UserRole account
        self.user_role.controller = *self.controller.to_account_info().key;
        self.user_role.role_name = role_name;
        self.user_role.actions = actions;
        self.user_role.users = Vec::new();
        self.user_role.bump = bump;
        Ok(())
    }
}

impl<'info> AssignUserToRole<'info> {
    pub fn assign_user_to_role(
        &mut self,
        user: Pubkey,
        user_roles: Vec<Account<UserRole>>,
    ) -> Result<()> {
        let admin = &self.admin;
        let controller = &self.controller;
        let role = &mut self.user_role;

        // Allow default admin OR someone with the "assign" permission
        let authorized = admin.key() == controller.default_admin
            || has_permission(&user_roles, &admin.key(), "assign");

        require!(authorized, ErrorCode::Unauthorized);

        //  Prevent duplicates
        require!(!role.users.contains(&user), ErrorCode::UserAlreadyAssigned);

        //  Assign user
        role.users.push(user);

        Ok(())
    }
}
