
#[cfg(not(feature = "client"))]
// use crate::events::{GroupEventHeader, LendingPoolBankAccrueInterestEvent};
use crate::{
    assert_struct_align, assert_struct_size, constants::MAX_ORACLE_KEYS
};
use anchor_lang::prelude::*;
use fixed::types::I80F48;
use super::price::OracleSetup;
use std::fmt::{Debug, Formatter};

#[cfg(feature = "client")]
use std::fmt::Display;
#[cfg(any(feature = "test", feature = "client"))]
use type_layout::TypeLayout;


#[account(zero_copy)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(Debug, PartialEq, Eq, TypeLayout)
)]
#[derive(Default)]
pub struct MarginfiGroup {
    pub admin: Pubkey,
    pub _padding_0: [[u64; 2]; 32],
    pub _padding_1: [[u64; 2]; 32],
}


#[cfg_attr(any(feature = "test", feature = "client"), derive(TypeLayout))]
#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug, Clone)]
pub struct GroupConfig {
    pub admin: Option<Pubkey>,
}

#[zero_copy]
#[repr(C)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(PartialEq, Eq, TypeLayout)
)]
#[derive(Default, Debug, AnchorDeserialize, AnchorSerialize)]
pub struct InterestRateConfig {
    // Curve Params
    pub optimal_utilization_rate: WrappedI80F48,
    pub plateau_interest_rate: WrappedI80F48,
    pub max_interest_rate: WrappedI80F48,

    // Fees
    pub insurance_fee_fixed_apr: WrappedI80F48,
    pub insurance_ir_fee: WrappedI80F48,
    pub protocol_fixed_fee_apr: WrappedI80F48,
    pub protocol_ir_fee: WrappedI80F48,

    pub _padding: [[u64; 2]; 8], // 16 * 8 = 128 bytes
}

assert_struct_size!(Bank, 1856);
assert_struct_align!(Bank, 8);
#[account(zero_copy(unsafe))]
#[repr(C)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(Debug, PartialEq, Eq, TypeLayout)
)]
#[derive(Default)]
pub struct Bank {
    pub mint: Pubkey,
    pub mint_decimals: u8,

    pub group: Pubkey,

    pub asset_share_value: WrappedI80F48,
    pub liability_share_value: WrappedI80F48,

    pub liquidity_vault: Pubkey,
    pub liquidity_vault_bump: u8,
    pub liquidity_vault_authority_bump: u8,

    pub insurance_vault: Pubkey,
    pub insurance_vault_bump: u8,
    pub insurance_vault_authority_bump: u8,
    pub collected_insurance_fees_outstanding: WrappedI80F48,

    pub fee_vault: Pubkey,
    pub fee_vault_bump: u8,
    pub fee_vault_authority_bump: u8,
    pub collected_group_fees_outstanding: WrappedI80F48,

    pub total_liability_shares: WrappedI80F48,
    pub total_asset_shares: WrappedI80F48,

    pub last_update: i64,

    pub config: BankConfig,

    /// Bank Config Flags
    ///
    /// - EMISSIONS_FLAG_BORROW_ACTIVE: 1
    /// - EMISSIONS_FLAG_LENDING_ACTIVE: 2
    /// - PERMISSIONLESS_BAD_DEBT_SETTLEMENT: 4
    ///
    pub flags: u64,
    /// Emissions APR.
    /// Number of emitted tokens (emissions_mint) per 1e(bank.mint_decimal) tokens (bank mint) (native amount) per 1 YEAR.
    pub emissions_rate: u64,
    pub emissions_remaining: WrappedI80F48,
    pub emissions_mint: Pubkey,

    pub _padding_0: [[u64; 2]; 28],
    pub _padding_1: [[u64; 2]; 32], // 16 * 2 * 32 = 1024B
}


#[repr(u8)]
#[cfg_attr(any(feature = "test", feature = "client"), derive(PartialEq, Eq))]
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, Default)]
pub enum BankOperationalState {
    Paused,
    #[default] Operational,
    ReduceOnly,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Default)]
pub enum RiskTier {
    #[default] Collateral,
    /// ## Isolated Risk
    /// Assets in this trance can be borrowed only in isolation.
    /// They can't be borrowed together with other assets.
    ///
    /// For example, if users has USDC, and wants to borrow XYZ which is isolated,
    /// they can't borrow XYZ together with SOL, only XYZ alone.
    Isolated,
}

assert_struct_size!(BankConfig, 544);
assert_struct_align!(BankConfig, 8);
#[zero_copy(unsafe)]
#[repr(C)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(PartialEq, Eq, TypeLayout)
)]
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Default)]
/// TODO: Convert weights to (u64, u64) to avoid precision loss (maybe?)
pub struct BankConfig {
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,

    pub liability_weight_init: WrappedI80F48,
    pub liability_weight_maint: WrappedI80F48,

    pub deposit_limit: u64,

    pub interest_rate_config: InterestRateConfig,
    pub operational_state: BankOperationalState,

    pub oracle_setup: OracleSetup,
    pub oracle_keys: [Pubkey; MAX_ORACLE_KEYS],

    pub borrow_limit: u64,

    pub risk_tier: RiskTier,

    /// USD denominated limit for calculating asset value for initialization margin requirements.
    /// Example, if total SOL deposits are equal to $1M and the limit it set to $500K,
    /// then SOL assets will be discounted by 50%.
    ///
    /// In other words the max value of liabilities that can be backed by the asset is $500K.
    /// This is useful for limiting the damage of orcale attacks.
    ///
    /// Value is UI USD value, for example value 100 -> $100
    pub total_asset_value_init_limit: u64,

    /// Time window in seconds for the oracle price feed to be considered live.
    pub oracle_max_age: u16,

    pub _padding: [u16; 19], // 16 * 4 = 64 bytes
}

#[zero_copy]
#[repr(C, align(8))]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(PartialEq, Eq, TypeLayout)
)]
#[derive(Default, AnchorDeserialize, AnchorSerialize)]
pub struct WrappedI80F48 {
    pub value: [u8; 16],
}

impl Debug for WrappedI80F48 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", I80F48::from_le_bytes(self.value))
    }
}

impl From<I80F48> for WrappedI80F48 {
    fn from(i: I80F48) -> Self {
        Self {
            value: i.to_le_bytes(),
        }
    }
}

impl From<WrappedI80F48> for I80F48 {
    fn from(w: WrappedI80F48) -> Self {
        Self::from_le_bytes(w.value)
    }
}
