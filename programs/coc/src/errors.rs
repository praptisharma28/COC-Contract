use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // access control errors
    #[msg("Controller has already been initialized.")]
    AlreadyInitialized,
    #[msg("Admin must be the signer.")]
    AdminMustSign,

    #[msg("Role name cannot be empty.")]
    InvalidRoleName,
    #[msg("User is already assigned with the role")]
    UserAlreadyAssigned,

    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Insufficient permissions for this action")]
    InsufficientPermissions,
    #[msg("Token is not active")]
    TokenNotActive,
    #[msg("Industry is not active")]
    IndustryNotActive,
    #[msg("Auction is not active")]
    AuctionNotActive,
    #[msg("Auction has expired")]
    AuctionExpired,
    #[msg("Insufficient tokens available in auction")]
    InsufficientTokensAvailable,
}
