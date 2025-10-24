// All the utility functions that support the main logic of your program is here

use crate::states::UserRole;
use anchor_lang::prelude::*;

pub fn has_permission(user_roles: &Vec<Account<UserRole>>, user: &Pubkey, action: &str) -> bool {
    for role in user_roles {
        if role.users.contains(user) && role.actions.contains(&action.to_string()) {
            return true;
        }
    }
    false
}
