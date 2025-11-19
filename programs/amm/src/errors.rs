use anchor_lang::prelude::*;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError{
    #[msg("Default error")]
    DefaultError,
    #[msg("Offer expired")]
    OfferExpired,
    #[msg("This pool is locked")]
    PoolLocked,
    #[msg("Slippage exceeded")]
    SlipageExceeded,
    #[msg("Overflow detected")]
    Overflow,
    #[msg("Underflow detected")]
    Underflow,
    #[msg["Invalid Token"]]
    InvalidToken,
    #[msg("Actual liquidity is less than minimum.")]
    LiquidityLessThanMinimum,
    #[msg("No liquidity in pool.")]
    NoLiquidityInPool,
    #[msg("Bump error.")]
    BumpError,
    #[msg("Curve Error")]
    CurveError,
    #[msg("Fee is greater than 100%. This is not a very good deal.")]
    InvalidFee,
    #[msg("Invalid update authority")]
    InvalidAuthority,
    #[msg("No update authority set.")]
    NoAuthoritySet,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Invalid precision")]
    InvalidPrecision,
    #[msg("Isufficient balance.")]
    InsufficientBalance,
    #[msg("Zero Balance.")]
    ZeroBalance,
}

fn from(error: CurveError) -> AmmError {
    match error {
        CurveError::InvalidPrecision => AmmError::InvalidPrecision,
        CurveError::Overflow => AmmError::Overflow,
        CurveError::Underflow => AmmError::Underflow,
        CurveError::InvalidFeeAmount => AmmError::InvalidFeeAmount,
        CurveError::InsufficientBalance => AmmError::InsufficientBalance,
        CurveError::ZeroBalance => AmmError::ZeroBalance,
        CurveError::SlippageLimitExceeded => AmmError::SlipageExceeded,
    }
}