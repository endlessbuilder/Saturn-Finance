use anchor_lang::prelude::*;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::constants::*;

// Performance fee when rebalancing
pub const PERFORMANCE_FEE_NUMERATOR: u128 = 500u128; // 5%
pub const PERFORMANCE_FEE_DENOMINATOR: u128 = 10000u128;

pub const MAX_STRATEGY: usize = 30;
pub const MAX_BUMPS: usize = 10;
pub const LOCKED_PROFIT_DEGRADATION_DENOMINATOR: u128 = 1_000_000_000_000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct VaultBumps {
    pub vault_bump: u8,
    pub token_vault_bump: u8,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Eq,
    Hash,
)]
pub enum StrategyType {
    PortFinanceWithoutLM,
    PortFinanceWithLM,
    SolendWithoutLM,
    Mango, // Mango is no longer supported
    SolendWithLM,
    ApricotWithoutLM,
    Francium,
    Tulip,
    // This is for compatibility with some administrative endpoint
    Vault,
    Drift,
    Frakt,
    Marginfi,
    Kamino,
}

impl std::fmt::Display for StrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub fn get_base_key() -> Pubkey {
    Pubkey::from_str("HWzXGcGHy4tcpYfaRDCyLNzXqBTv3E6BttpCH2vJxArv").unwrap()
}

/// Treasury address
pub fn get_treasury_address() -> Pubkey {
    Pubkey::from_str("9kZeN47U2dubGbbzMrzzoRAUvpuxVLRcjW9XiFpYjUo4").unwrap()
}

pub fn get_base_address_for_idle_vault() -> Pubkey {
    Pubkey::default()
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct LockedProfitTracker {
    pub last_updated_locked_profit: u64,
    pub last_report: u64,
    pub locked_profit_degradation: u64,
}

impl Default for LockedProfitTracker {
    fn default() -> Self {
        LockedProfitTracker {
            last_updated_locked_profit: 0,
            last_report: 0,
            locked_profit_degradation: u64::try_from(LOCKED_PROFIT_DEGRADATION_DENOMINATOR)
                .unwrap()
                / (6 * 3600), // locked profit is fully dripped in 6 hour
        }
    }
}
impl LockedProfitTracker {
    // based from yearn vault
    // https://github.com/yearn/yearn-vaults/blob/main/contracts/Vault.vy#L825
    pub fn calculate_locked_profit(&self, current_time: u64) -> Option<u64> {
        let duration = u128::from(current_time.checked_sub(self.last_report)?);
        let locked_profit_degradation = u128::from(self.locked_profit_degradation);
        let locked_fund_ratio = duration.checked_mul(locked_profit_degradation)?;

        if locked_fund_ratio > LOCKED_PROFIT_DEGRADATION_DENOMINATOR {
            return Some(0);
        }
        let locked_profit = u128::from(self.last_updated_locked_profit);

        let locked_profit = (locked_profit
            .checked_mul(LOCKED_PROFIT_DEGRADATION_DENOMINATOR - locked_fund_ratio)?)
        .checked_div(LOCKED_PROFIT_DEGRADATION_DENOMINATOR)?;
        let locked_profit = u64::try_from(locked_profit).ok()?;
        Some(locked_profit)
    }
}

