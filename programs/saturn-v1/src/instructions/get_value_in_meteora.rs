use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use dynamic_amm::constants::virtual_price;

use crate::error::*;
use crate::account::Treasury;
use crate::constants::{TREASURY_AUTHORITY_SEED, TREASURY_SEED, TREASURY_METEORA_LP, PRICE_PRECISION};
use dynamic_amm::state::Pool;
use dynamic_vault::state::Vault;

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

    /// CHECK: Vault account (PDA)
    #[account(
        mut,
        constraint = a_vault.key() == pool.a_vault,
    )]
    pub a_vault: Account<'info, Vault>,

    /// CHECK: Vault account (PDA)
    #[account(
        mut,
        constraint = b_vault.key() == pool.b_vault,
    )]
    pub b_vault: Account<'info, Vault>,

    #[account(
        mut,
        constraint = user_pool_lp.mint.key() == pool.lp_mint,
    )]
    pub user_pool_lp: Account<'info, TokenAccount>

}

pub fn handle(ctx: Context<GetValueInMeteora>) -> Result<f64> {
    let pool = &mut ctx.accounts.pool;
    let user_pool_lp = &mut ctx.accounts.user_pool_lp;
    let a_vault = &mut ctx.accounts.a_vault;
    let b_vault = &mut ctx.accounts.b_vault;

    let current_time = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
    let a_lp_amount = u128::from(a_vault.get_unlocked_amount(current_time).unwrap()).checked_mul(PRICE_PRECISION).unwrap();
    let b_lp_amount = u128::from(a_vault.get_unlocked_amount(current_time).unwrap()).checked_mul(PRICE_PRECISION).unwrap();
    let virtual_price = (a_lp_amount + b_lp_amount).checked_div(u128::from(pool.total_locked_lp)).unwrap();

    let value = (user_pool_lp.amount * u64::try_from(virtual_price).unwrap()) as f64;
    Ok(value)
}