use anchor_lang::prelude::*;

#[error_code]
pub enum BondError {
    #[msg("Insufficient Funds")]
    InsufficientFundsError,
    #[msg("Token Mint Error")]
    TokenMintError,
    #[msg("Get Back Price Error")]
    BackPriceError,
    #[msg("Get Spot Price Error")]
    SpotPriceError,
    #[msg("Get Deduction Error")]
    DeductionError,
    #[msg("Treasury Fund Error")]
    TreasuryFundError,
    #[msg("Collateral Not in List Error")]
    CollateralError,
    #[msg("Bond Not finished")]
    BondNotFinished,
    #[msg("Not the Creator")]
    CreatorError,
    #[msg("Already Redeemed")]
    AlreadyRedeem,
    #[msg("Not enough staked Saturn To Unstake")]
    UnstakingError,
    #[msg("IncorrectOwner")]
    IncorrectOwner
}

// Meteora error
#[error_code]
pub enum VaultError {
    #[msg("Vault is disabled")]
    VaultIsDisabled,

    #[msg("Exceeded slippage tolerance")]
    ExceededSlippage,

    #[msg("Strategy is not existed")]
    StrategyIsNotExisted,

    #[msg("UnAuthorized")]
    UnAuthorized,

    #[msg("Math operation overflow")]
    MathOverflow,

    #[msg("Protocol is not supported")]
    ProtocolIsNotSupported,

    #[msg("Reserve does not support token mint")]
    UnMatchReserve,

    #[msg("lockedProfitDegradation is invalid")]
    InvalidLockedProfitDegradation,

    #[msg("Maximum number of strategies have been reached")]
    MaxStrategyReached,

    #[msg("Strategy existed")]
    StrategyExisted,

    #[msg("Invalid unmint amount")]
    InvalidUnmintAmount,

    #[msg("Invalid accounts for strategy")]
    InvalidAccountsForStrategy,

    #[msg("Invalid bump")]
    InvalidBump,

    #[msg("Amount must be greater than 0")]
    AmountMustGreaterThanZero,

    #[msg("Mango is not supported anymore")]
    MangoIsNotSupportedAnymore,

    #[msg("Strategy is not supported")]
    StrategyIsNotSupported,

    #[msg("Pay amount is exceeded")]
    PayAmountIsExeeced,
}
