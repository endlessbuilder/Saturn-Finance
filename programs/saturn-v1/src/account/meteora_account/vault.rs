use anchor_lang::prelude::*;
use std::convert::TryFrom;
use std::fmt::Debug;

use crate::meteora_utils::MAX_STRATEGY;
use crate::meteora_utils::LockedProfitTracker;

#[account]
#[derive(Default, Debug)]
pub struct Vault {
    pub enabled: u8,
    pub bumps: VaultBumps,

    pub total_amount: u64,

    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub token_mint: Pubkey,

    pub lp_mint: Pubkey,
    pub strategies: [Pubkey; MAX_STRATEGY],

    pub base: Pubkey,
    pub admin: Pubkey,
    pub operator: Pubkey, // person to send crank
    pub locked_profit_tracker: LockedProfitTracker,
}

impl Vault {
    pub fn get_unlocked_amount(&self, current_time: u64) -> Option<u64> {
        self.total_amount.checked_sub(
            self.locked_profit_tracker
                .calculate_locked_profit(current_time)?,
        )
    }

    pub fn get_amount_by_share(
        &self,
        current_time: u64,
        share: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(share)
                .checked_mul(u128::from(total_amount))?
                .checked_div(u128::from(total_supply))?,
        )
        .ok()
    }

    pub fn get_unmint_amount(
        &self,
        current_time: u64,
        out_token: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(out_token)
                .checked_mul(u128::from(total_supply))?
                .checked_div(u128::from(total_amount))?,
        )
        .ok()
    }

    pub fn is_strategy_existed(&self, pubkey: Pubkey) -> bool {
        for item in self.strategies.iter() {
            if *item == pubkey {
                return true;
            }
        }
        false
    }
}

pub struct VaultBumps {
    pub vault_bump: u8,
    pub token_vault_bump: u8,
}