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

declare_id!("BfgBHPt6wALzb5JK3x95RfuJHZAA658pqRswutqWMLzv");

#[program]
pub mod saturn_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    pub fn apply_bond(ctx: Context<ApplyBond>, args: ApplyBondArgs) -> Result<()> {
        instructions::apply_bond::handle(ctx, args)        
    }

    pub fn finish_bond(ctx: Context<FinishBond>) -> Result<()> {
        instructions::finish_bond::handle(ctx)        
    }
}

