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

declare_id!("GSQceBxFJCJBd4Wo5enf9GB4Qr1nXcXjfATbMR8EUqmK");

#[program]
pub mod saturn_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    pub fn apply_bond(ctx: Context<ApplyBond>, token_amount: u64, spot_price: u64, bump: u8) -> Result<()> {
        instructions::apply_bond::handle(ctx, token_amount, spot_price, bump)        
    }

    pub fn finish_bond(ctx: Context<FinishBond>, bump:u8) -> Result<()> {
        instructions::finish_bond::handle(ctx, bump)        
    }
}

