#![allow(unused_imports)]
#![allow(unused_variables)]
use anchor_lang::prelude::*;

mod account;
mod instructions;
mod constants;
mod utils;

use account::*;
use instructions::*;

declare_id!("5mWSmPkAEesVq134hxA1gqFiwDXLArUacKXfmbmXwBBt");

#[program]
pub mod saturn_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    pub fn apply_bond(ctx: Context<ApplyBond>) -> Result<()> {
        instructions::apply_bond::handle(ctx)        
    }
}

