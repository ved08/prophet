use anchor_lang::prelude::*;

#[error_code]
pub enum MarketErrors {
    #[msg("Market expired cannot bet now")]
    MarketExpiredError,
    #[msg("Amount cannot be negative")]
    AmountCannotBeNegative,
}
