use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::error::*;
use crate::account::Treasury;
use crate::constants::{TREASURY_AUTHORITY_SEED, TREASURY_SEED, TREASURY_METEORA_LP};
use dynamic_amm::state::Pool;

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

    /// CHECK: Pool account (PDA)
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        constraint = user_pool_lp.mint.key() == pool.lp_mint,
    )]
    pub user_pool_lp: Account<'info, TokenAccount>

}

pub fn handle(ctx: Context<GetValueInMeteora>) -> Result<u64> {
    let pool = ctx.accounts.pool;
    let user_pool_lp = &mut ctx.accounts.user_pool_lp;
        

   Ok(0)
}