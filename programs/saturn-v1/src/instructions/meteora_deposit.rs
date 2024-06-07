use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::meteora_utils::*;
use crate::account::meteora_account::Vault;

#[derive(Accounts)]
pub struct MeteoraDeposit<'info> {
    #[account(
        mut,
        has_one = token_vault,
        has_one = lp_mint,
    )]
    pub vault: Box<Account<'info, Vault>>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[allow(unused_variables)]
pub fn handle(
    ctx: Context<MeteoraDeposit>,
    token_amount: u64,
    minimum_lp_token_amount: u64,
) -> Result<()> {
    Ok(())
}