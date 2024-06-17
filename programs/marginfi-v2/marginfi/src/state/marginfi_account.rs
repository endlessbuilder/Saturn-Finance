use super::marginfi_group::WrappedI80F48;
use crate::{
    assert_struct_align, assert_struct_size,
};
use anchor_lang::prelude::*;

#[cfg(any(feature = "test", feature = "client"))]
use type_layout::TypeLayout;

assert_struct_size!(MarginfiAccount, 2304);
assert_struct_align!(MarginfiAccount, 8);
#[account(zero_copy(unsafe))]
#[repr(C)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(Debug, PartialEq, Eq, TypeLayout)
)]
pub struct MarginfiAccount {
    pub group: Pubkey,                   // 32
    pub authority: Pubkey,               // 32
    pub lending_account: LendingAccount, // 1728
    /// The flags that indicate the state of the account.
    /// This is u64 bitfield, where each bit represents a flag.
    ///
    /// Flags:
    /// - DISABLED_FLAG = 1 << 0 = 1 - This flag indicates that the account is disabled,
    /// and no further actions can be taken on it.
    pub account_flags: u64, // 8
    pub _padding: [u64; 63],             // 8 * 63 = 512
}

pub const DISABLED_FLAG: u64 = 1 << 0;
pub const IN_FLASHLOAN_FLAG: u64 = 1 << 1;
pub const FLASHLOAN_ENABLED_FLAG: u64 = 1 << 2;
pub const TRANSFER_AUTHORITY_ALLOWED_FLAG: u64 = 1 << 3;
pub const MAX_LENDING_ACCOUNT_BALANCES: usize = 16;

assert_struct_size!(LendingAccount, 1728);
assert_struct_align!(LendingAccount, 8);
#[zero_copy(unsafe)]
#[repr(C)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(Debug, PartialEq, Eq, TypeLayout)
)]
pub struct LendingAccount {
    pub balances: [Balance; MAX_LENDING_ACCOUNT_BALANCES], // 104 * 16 = 1664
    pub _padding: [u64; 8],                                // 8 * 8 = 64
}

assert_struct_size!(Balance, 104);
assert_struct_align!(Balance, 8);
#[zero_copy(unsafe)]
#[repr(C)]
#[cfg_attr(
    any(feature = "test", feature = "client"),
    derive(Debug, PartialEq, Eq, TypeLayout)
)]
pub struct Balance {
    pub active: bool,
    pub bank_pk: Pubkey,
    pub asset_shares: WrappedI80F48,
    pub liability_shares: WrappedI80F48,
    pub emissions_outstanding: WrappedI80F48,
    pub last_update: u64,
    pub _padding: [u64; 1],
}

