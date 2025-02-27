use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Below Min Health Factor")]
    BelowMinHealthFactor,
    #[msg("Cannot liquidate healthy account")]
    AboveMinHealthFactor
}
