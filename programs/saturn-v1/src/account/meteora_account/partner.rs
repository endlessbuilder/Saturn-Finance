use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

/// Partner struct
#[account]
#[derive(Debug)]
pub struct Partner {
    /// partner token address, which is used to get fee later (fee is in native token)
    pub partner_token: Pubkey, // 32
    /// vault address that partner integrates
    pub vault: Pubkey, // 32
    /// total fee that partner get, but haven't sent yet
    pub outstanding_fee: u64, // 8
    /// fee ratio partner get in performance fee
    pub fee_ratio: u64, // 8
    // cumulative fee partner get from start
    pub cumulative_fee: u128, // 16
}

impl Partner {
    /// accrue fee
    pub fn accrue_fee(&mut self, fee: u64) -> Option<()> {
        self.outstanding_fee = self.outstanding_fee.checked_add(fee)?;
        let max = u128::MAX;
        let buffer = max - self.cumulative_fee;
        let fee: u128 = fee.into();
        if buffer >= fee {
            // only add if we have enough room
            self.cumulative_fee = self.cumulative_fee.checked_add(fee)?;
        }
        Some(())
    }
}