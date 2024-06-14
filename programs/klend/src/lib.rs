#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod lending_market;
pub mod state;
pub mod utils;
pub mod context;

use utils::constraints::emergency_mode_disabled;
use context::*;
pub use crate::{state::*, utils::fraction};

#[cfg(feature = "staging")]
declare_id!("SLendK7ySfcEzyaFqy93gDnD3RtrpXJcnRwb6zFHJSh");

#[cfg(not(feature = "staging"))]
declare_id!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "Kamino Lending",
    project_url: "https://kamino.finance/",
    contacts: "email:security@kamino.finance",
    policy: "https://github.com/hubbleprotocol/audits/blob/master/docs/SECURITY.md",

       preferred_languages: "en",
    auditors: "OtterSec, Offside Labs"
}

#[program]
pub mod kamino_lending {

    use super::*;

    pub fn init_user_metadata(
        ctx: Context<InitUserMetadata>,
        user_lookup_table: Pubkey,
    ) -> Result<()> {
        // handler_init_user_metadata::process(ctx, user_lookup_table)
        Ok(())
    }

    pub fn init_obligation(ctx: Context<InitObligation>, args: InitObligationArgs) -> Result<()> {
        // handler_init_obligation::process(ctx, args)
        Ok(())
    }

    #[access_control(emergency_mode_disabled(&ctx.accounts.lending_market))]
    pub fn deposit_reserve_liquidity_and_obligation_collateral(
        ctx: Context<DepositReserveLiquidityAndObligationCollateral>,
        liquidity_amount: u64,
    ) -> Result<()> {
        // handler_deposit_reserve_liquidity_and_obligation_collateral::process(ctx, liquidity_amount)
        Ok(())
    }

    #[access_control(emergency_mode_disabled(&ctx.accounts.lending_market))]
    pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral(
        ctx: Context<WithdrawObligationCollateralAndRedeemReserveCollateral>,
        collateral_amount: u64,
    ) -> Result<()> {
        // handler_withdraw_obligation_collateral_and_redeem_reserve_collateral::process(
        //     ctx,
        //     collateral_amount,
        // )
        Ok(())
    }
    
}

#[error_code]
#[derive(PartialEq, Eq, strum::EnumString)]
pub enum LendingError {
    #[msg("Market authority is invalid")]
    InvalidMarketAuthority,
    #[msg("Market owner is invalid")]
    InvalidMarketOwner,
    #[msg("Input account owner is not the program address")]
    InvalidAccountOwner,
    #[msg("Input amount is invalid")]
    InvalidAmount,
    #[msg("Input config value is invalid")]
    InvalidConfig,
    #[msg("Input account must be a signer")]
    InvalidSigner,
    #[msg("Invalid account input")]
    InvalidAccountInput,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Insufficient liquidity available")]
    InsufficientLiquidity,
    #[msg("Reserve state needs to be refreshed")]
    ReserveStale,
    #[msg("Withdraw amount too small")]
    WithdrawTooSmall,
    #[msg("Withdraw amount too large")]
    WithdrawTooLarge,
    #[msg("Borrow amount too small to receive liquidity after fees")]
    BorrowTooSmall,
    #[msg("Borrow amount too large for deposited collateral")]
    BorrowTooLarge,
    #[msg("Repay amount too small to transfer liquidity")]
    RepayTooSmall,
    #[msg("Liquidation amount too small to receive collateral")]
    LiquidationTooSmall,
    #[msg("Cannot liquidate healthy obligations")]
    ObligationHealthy,
    #[msg("Obligation state needs to be refreshed")]
    ObligationStale,
    #[msg("Obligation reserve limit exceeded")]
    ObligationReserveLimit,
    #[msg("Obligation owner is invalid")]
    InvalidObligationOwner,
    #[msg("Obligation deposits are empty")]
    ObligationDepositsEmpty,
    #[msg("Obligation borrows are empty")]
    ObligationBorrowsEmpty,
    #[msg("Obligation deposits have zero value")]
    ObligationDepositsZero,
    #[msg("Obligation borrows have zero value")]
    ObligationBorrowsZero,
    #[msg("Invalid obligation collateral")]
    InvalidObligationCollateral,
    #[msg("Invalid obligation liquidity")]
    InvalidObligationLiquidity,
    #[msg("Obligation collateral is empty")]
    ObligationCollateralEmpty,
    #[msg("Obligation liquidity is empty")]
    ObligationLiquidityEmpty,
    #[msg("Interest rate is negative")]
    NegativeInterestRate,
    #[msg("Input oracle config is invalid")]
    InvalidOracleConfig,
    #[msg("Insufficient protocol fees to claim or no liquidity available")]
    InsufficientProtocolFeesToRedeem,
    #[msg("No cpi flash borrows allowed")]
    FlashBorrowCpi,
    #[msg("No corresponding repay found for flash borrow")]
    NoFlashRepayFound,
    #[msg("Invalid repay found")]
    InvalidFlashRepay,
    #[msg("No cpi flash repays allowed")]
    FlashRepayCpi,
    #[msg("Multiple flash borrows not allowed in the same transaction")]
    MultipleFlashBorrows,
    #[msg("Flash loans are disabled for this reserve")]
    FlashLoansDisabled,
    #[msg("Switchboard error")]
    SwitchboardV2Error,
    #[msg("Cannot deserialize the scope price account")]
    CouldNotDeserializeScope,
    #[msg("Price too old")]
    PriceTooOld,
    #[msg("Price too divergent from twap")]
    PriceTooDivergentFromTwap,
    #[msg("Invalid twap price")]
    InvalidTwapPrice,
    #[msg("Emergency mode is enabled")]
    GlobalEmergencyMode,
    #[msg("Invalid lending market config")]
    InvalidFlag,
    #[msg("Price is not valid")]
    PriceNotValid,
    #[msg("Price is bigger than allowed by heuristic")]
    PriceIsBiggerThanHeuristic,
    #[msg("Price lower than allowed by heuristic")]
    PriceIsLowerThanHeuristic,
    #[msg("Price is zero")]
    PriceIsZero,
    #[msg("Price confidence too wide")]
    PriceConfidenceTooWide,
    #[msg("Conversion between integers failed")]
    IntegerOverflow,
    #[msg("This reserve does not have a farm")]
    NoFarmForReserve,
    #[msg("Wrong instruction at expected position")]
    IncorrectInstructionInPosition,
    #[msg("No price found")]
    NoPriceFound,
    #[msg("Invalid Twap configuration: Twap is enabled but one of the enabled price doesn't have a twap")]
    InvalidTwapConfig,
    #[msg("Pyth price account does not match configuration")]
    InvalidPythPriceAccount,
    #[msg("Switchboard account(s) do not match configuration")]
    InvalidSwitchboardAccount,
    #[msg("Scope price account does not match configuration")]
    InvalidScopePriceAccount,
    #[msg("The obligation has one collateral with an LTV set to 0. Withdraw it before withdrawing other collaterals")]
    ObligationCollateralLtvZero,
    #[msg("Seeds must be default pubkeys for tag 0, and mint addresses for tag 1 or 2")]
    InvalidObligationSeedsValue,
    #[msg("Obligation id must be 0")]
    InvalidObligationId,
    #[msg("Invalid borrow rate curve point")]
    InvalidBorrowRateCurvePoint,
    #[msg("Invalid utilization rate")]
    InvalidUtilizationRate,
    #[msg("Obligation hasn't been fully liquidated and debt cannot be socialized.")]
    CannotSocializeObligationWithCollateral,
    #[msg("Obligation has no borrows or deposits.")]
    ObligationEmpty,
    #[msg("Withdrawal cap is reached")]
    WithdrawalCapReached,
    #[msg("The last interval start timestamp is greater than the current timestamp")]
    LastTimestampGreaterThanCurrent,
    #[msg("The reward amount is less than the minimum acceptable received collateral")]
    LiquidationSlippageError,
    #[msg("Isolated Asset Tier Violation")]
    IsolatedAssetTierViolation,
    #[msg("The obligation's elevation group and the reserve's are not the same")]
    InconsistentElevationGroup,
    #[msg("The elevation group chosen for the reserve does not exist in the lending market")]
    InvalidElevationGroup,
    #[msg("The elevation group updated has wrong parameters set")]
    InvalidElevationGroupConfig,
    #[msg("The current obligation must have most or all its debt repaid before changing the elevation group")]
    UnhealthyElevationGroupLtv,
    #[msg("Elevation group does not accept any new loans or any new borrows/withdrawals")]
    ElevationGroupNewLoansDisabled,
    #[msg("Reserve was deprecated, no longer usable")]
    ReserveDeprecated,
    #[msg("Referrer account not initialized")]
    ReferrerAccountNotInitialized,
    #[msg("Referrer account mint does not match the operation reserve mint")]
    ReferrerAccountMintMissmatch,
    #[msg("Referrer account address is not a valid program address")]
    ReferrerAccountWrongAddress,
    #[msg("Referrer account referrer does not match the owner referrer")]
    ReferrerAccountReferrerMissmatch,
    #[msg("Referrer account missing for obligation with referrer")]
    ReferrerAccountMissing,
    #[msg("Insufficient referral fees to claim or no liquidity available")]
    InsufficientReferralFeesToRedeem,
    #[msg("CPI disabled for this instruction")]
    CpiDisabled,
    #[msg("Referrer short_url is not ascii alphanumeric")]
    ShortUrlNotAsciiAlphanumeric,
    #[msg("Reserve is marked as obsolete")]
    ReserveObsolete,
    #[msg("Obligation already part of the same elevation group")]
    ElevationGroupAlreadyActivated,
    #[msg("Obligation has a deposit in a deprecated reserve")]
    ObligationInDeprecatedReserve,
    #[msg("Referrer state owner does not match the given signer")]
    ReferrerStateOwnerMismatch,
    #[msg("User metadata owner is already set")]
    UserMetadataOwnerAlreadySet,
    #[msg("This collateral cannot be liquidated (LTV set to 0)")]
    CollateralNonLiquidatable,
    #[msg("Borrowing is disabled")]
    BorrowingDisabled,
    #[msg("Cannot borrow above borrow limit")]
    BorrowLimitExceeded,
    #[msg("Cannot deposit above deposit limit")]
    DepositLimitExceeded,
    #[msg("Reserve does not accept any new borrows outside elevation group")]
    BorrowingDisabledOutsideElevationGroup,
    #[msg("Net value remaining too small")]
    NetValueRemainingTooSmall,
    #[msg("Cannot get the obligation in a worse position")]
    WorseLTVBlocked,
    #[msg("Cannot have more liabilities than assets in a position")]
    LiabilitiesBiggerThanAssets,
    #[msg("Reserve state and token account cannot drift")]
    ReserveTokenBalanceMismatch,
    #[msg("Reserve token account has been unexpectedly modified")]
    ReserveVaultBalanceMismatch,
    #[msg("Reserve internal state accounting has been unexpectedly modified")]
    ReserveAccountingMismatch,
    #[msg("Borrowing above set utilization rate is disabled")]
    BorrowingAboveUtilizationRateDisabled,
}

pub type LendingResult<T = ()> = std::result::Result<T, LendingError>;
