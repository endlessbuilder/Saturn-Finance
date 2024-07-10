use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::error::*;
use crate::account::Treasury;
use crate::constants::{TREASURY_AUTHORITY_SEED, TREASURY_METEORA_LP};
use meteora::state::Vault;

#[derive(Accounts)]
pub struct GetValueInMeteora<'info> {
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

pub fn handle(ctx: Context<GetValueInMeteora>) -> Result<u64> {

//     // # get meteora vault value
//    let treasury_authority = &mut ctx.accounts.treasury_authority;
//    let treasury_lp = &mut ctx.accounts.treasury_lp;
//    let meteora_vault = &mut ctx.accounts.meteora_vault;
//    let vault_lp_mint = &mut ctx.accounts.vault_lp_mint;
   
//     let current_time = u64::try_from(Clock::get()?.unix_timestamp)
//         .ok()
//         .ok_or(VaultError::MathOverflow)?;

//     let virtual_meteora_price = meteora_vault
//         .get_virtual_price(current_time, vault_lp_mint.supply)
//         .ok_or(VaultError::MathOverflow)?;

//     let value_in_meteora: u64 = treasury_lp.amount * virtual_meteora_price.into();

   Ok(0)
}