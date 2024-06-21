use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::{error::*, kamino_utils};
use crate::account::Treasury;
use crate::constants::{TREASURY_AUTHORITY_SEED, TREASURY_METEORA_LP};
use meteora::state::Vault;


#[derive(Accounts)]
pub struct GetState {
     /// CHECK:
     #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    /// treasury lp CHECK:
    #[account(
        mut, 
        constraint = treasury_lp.owner == treasury_authority.key(),
        seeds = [TREASURY_METEORA_LP.as_ref()],
        bump,
    )] //mint to account of treasury PDA
    pub treasury_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub vault_lp_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    #[account(mut)]
    pub meteora_vault: Box<Account<'info, Vault>>,

}

pub fn handle(ctx: Context<GetState>) -> Result<()> {
    // # get meteora vault value
   let treasury_authority = &mut ctx.accounts.treasury_authority;
   let treasury_lp = &mut ctx.accounts.treasury_lp;
   let meteora_vault = &mut ctx.accounts.meteora_vault;
   let vault_lp_mint = &mut ctx.accounts.vault_lp_mint;
   
    let current_time = u64::try_from(Clock::get()?.unix_timestamp)
        .ok()
        .ok_or(VaultError::MathOverflow)?;

    let virtual_meteora_price = meteora_vault
        .get_virtual_price(current_time, vault_lp_mint.supply)
        .ok_or(VaultError::MathOverflow)?;

    let value_in_meteora = treasury_lp.amount * virtual_meteora_price;

    // # get kamino value
    let clock = Clock::get();
    
    let withdraw_obligation_amount = kamino_utils::withdraw_obligation_collateral(
        lending_market,
        reserve,
        obligation,
        collateral_amount,
        clock.slot,
        ctx.accounts.withdraw_reserve.key(),
    )?;
    let withdraw_liquidity_amount = kamino_utils::redeem_reserve_collateral(
        reserve,
        withdraw_obligation_amount,
        clock,
        true,
    )?;

   Ok(())
}