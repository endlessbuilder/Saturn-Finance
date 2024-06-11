use anchor_lang::prelude::*;
use meteora::{
    PERFORMANCE_FEE_DENOMINATOR,
    PERFORMANCE_FEE_NUMERATOR,
};

use crate::constants::{
    FEE_DENOMINATOR,
    PRICE_PRECISION,
};

// User struct
#[account]
#[derive(Default, Debug)]
pub struct User {
    pub owner: Pubkey,
    /// partner address, each user can integrate with more partners
    pub partner: Pubkey,
    /// current virtual price
    pub current_virtual_price: u64,
    /// lp_token that user holds
    pub lp_token: u64,
    /// user bump
    pub bump: u8,
}

impl User {
    /// get fee per user
    pub fn get_fee(&mut self, virtual_price: u64, fee_ratio: u64) -> Option<u64> {
        if virtual_price <= self.current_virtual_price {
            // if virtual price is reduced, then no fee is accrued
            return Some(0);
        }
        let yield_earned = u128::from(self.lp_token)
            .checked_mul(u128::from(
                virtual_price.checked_sub(self.current_virtual_price)?,
            ))?
            .checked_div(PRICE_PRECISION)?;

        let performance_fee_by_vault = yield_earned
            .checked_mul(PERFORMANCE_FEE_NUMERATOR)?
            .checked_div(PERFORMANCE_FEE_DENOMINATOR)?;

        let fee_sharing = u64::try_from(
            performance_fee_by_vault
                .checked_mul(fee_ratio.into())?
                .checked_div(FEE_DENOMINATOR)?,
        )
        .ok()?;

        Some(fee_sharing)
    }

    /// set new state
    pub fn set_new_state(&mut self, virtual_price: u64, lp_token: u64) {
        self.current_virtual_price = virtual_price;
        self.lp_token = lp_token;
    }
}