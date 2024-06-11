use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::account::meteora_account::{Partner, User};
use crate::constants::{TOKEN_VAULT_PREFIX, VAULT_PREFIX};
// use crate::meteora_context::DepositWithdrawLiquidity;
use crate::meteora_utils::{MeteoraProgram, MeteoraUtils, update_liquidity_wrapper};
use meteora::state::Vault;


/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct MeteoraWithdraw<'info> {
    /// CHECK:
    #[account(mut, has_one = vault)]
    pub partner: Box<Account<'info, Partner>>,
    /// CHECK:
    #[account(mut, has_one = partner, has_one = owner)]
    pub user: Box<Account<'info, User>>,
    /// CHECK:
    pub vault_program: Program<'info, MeteoraProgram>,
    /// CHECK:
    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK:
    #[account(mut)]
    pub token_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault_lp_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    #[account(mut)]
    pub user_token: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut, constraint = user_lp.owner == user.key())] //mint to account of user PDA
    pub user_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub owner: Signer<'info>,
    /// CHECK:
    pub token_program: Program<'info, Token>,
}

 /// withdraw
 #[allow(clippy::needless_lifetimes)]
 pub fn handle<'a, 'b, 'c, 'info>(
     ctx: Context<'a, 'b, 'c, 'info, MeteoraWithdraw>,
     unmint_amount: u64,
     min_out_amount: u64,
 ) -> Result<()> {
     let partner_key = ctx.accounts.partner.key();
     let owner_key = ctx.accounts.owner.key();
     let user_seeds = &[
         partner_key.as_ref(),
         owner_key.as_ref(),
         &[ctx.accounts.user.bump],
     ];

     let vault = &ctx.accounts.vault.to_account_info();
     let user = &ctx.accounts.user.to_account_info();
     let vault_lp_mint = &ctx.accounts.vault_lp_mint.to_account_info();
     let user_lp = &ctx.accounts.user_lp.to_account_info();

     let user_token = &ctx.accounts.user_token.to_account_info();
     let token_vault = &ctx.accounts.token_vault.to_account_info();
     let token_program = &ctx.accounts.token_program.to_account_info();
     let vault_program = &ctx.accounts.vault_program.to_account_info();
     update_liquidity_wrapper(
         move || {
             MeteoraUtils::withdraw(
                 vault,
                 vault_lp_mint,
                 user_token,
                 user_lp,
                 user,
                 token_vault,
                 token_program,
                 vault_program,
                 unmint_amount,
                 min_out_amount,
                 &[&user_seeds[..]],
             )?;

             Ok(())
         },
         &mut ctx.accounts.vault,
         &mut ctx.accounts.vault_lp_mint,
         &mut ctx.accounts.user_lp,
         &mut ctx.accounts.partner,
         &mut ctx.accounts.user,
     )?;
     Ok(())
 }