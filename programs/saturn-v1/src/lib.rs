use anchor_lang::prelude::*;

declare_id!("5mWSmPkAEesVq134hxA1gqFiwDXLArUacKXfmbmXwBBt");

#[program]
pub mod saturn_v1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
