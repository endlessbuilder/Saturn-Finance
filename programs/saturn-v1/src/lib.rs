#![allow(unused_imports)]
#![allow(unused_variables)]
use anchor_lang::prelude::*;

mod account;
mod error;
mod instructions;
mod constants;
mod utils;

use account::*;
use instructions::*;

declare_id!("5mWSmPkAEesVq134hxA1gqFiwDXLArUacKXfmbmXwBBt");

#[program]
pub mod saturn_v_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    // Create Bond and Pay 
    pub fn apply_bond(ctx: Context<ApplyBond>, args: ApplyBondArgs) -> Result<()> {
        instructions::apply_bond::handle(ctx, args.token_amount, args.spot_price)        
    }

    // Redeem Bond after period is over.
    pub fn finish_bond(ctx: Context<FinishBond>) -> Result<()> {
        instructions::finish_bond::handle(ctx)        
    }

    // Amount to stake in STF token (9 Decimals)
    pub fn stake_stf(ctx: Context<StakeSTF>, amount_to_stake: u64) -> Result<()> {
        instructions::stake::handle(ctx, amount_to_stake)        
    }

    // Amount to unstake in staking index not in STF token
    pub fn unstake_stf(ctx: Context<UnStakeSTF>, amount_to_unstake: u64) -> Result<()> {
        instructions::unstake::handle(ctx, amount_to_unstake)        
    }

    pub fn swap_to_sol(ctx: Context<SwapToSOL>, data: Vec<u8>) -> Result<()> {
        instructions::swap_to_sol::handle(ctx, data)
    }
}

