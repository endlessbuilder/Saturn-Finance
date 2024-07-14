use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::error::*;
use crate::account::Treasury;
use crate::constants::{TREASURY_AUTHORITY_SEED, TREASURY_SEED, TREASURY_METEORA_LP};

#[derive(Accounts)]
pub struct GetValueInMeteora<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury_authority.key()
    )]
    signer: Signer<'info>,
     /// CHECK:
     #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    

}

pub fn handle(ctx: Context<GetValueInMeteora>) -> Result<u64> {

    

   Ok(0)
}