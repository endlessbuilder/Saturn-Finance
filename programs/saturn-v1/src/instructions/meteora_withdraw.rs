use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::account::meteora_account::Partner;
use crate::constants::{TOKEN_VAULT_PREFIX, TREASURY_AUTHORITY_SEED, VAULT_PREFIX};
// use crate::meteora_context::DepositWithdrawLiquidity;
use crate::meteora_utils::{MeteoraProgram, MeteoraUtils, update_liquidity_wrapper};
use meteora::state::Vault;


/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct MeteoraWithdraw<'info> {
    /// partner info CHECK:
    #[account(mut, has_one = vault)]
    pub partner: Box<Account<'info, Partner>>,
    /// treasury CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,
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
    /// treasury token CHECK:
    #[account(mut)]
    pub treasury_token: UncheckedAccount<'info>,
    /// treasury lp CHECK:
    #[account(mut, constraint = treasury_lp.owner == treasury_authority.key())] //mint to account of treasury PDA
    pub treasury_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    // pub owner: Signer<'info>,
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
     let owner_key = ctx.accounts.treasury_authority.key();
     let user_seeds = &[
         partner_key.as_ref(),
         owner_key.as_ref(),
         &[ctx.accounts.partner.bump],
     ];

     let vault = &ctx.accounts.vault.to_account_info();
    //  let user = &ctx.accounts.user.to_account_info();
     let vault_lp_mint = &ctx.accounts.vault_lp_mint.to_account_info();
     let treasury_lp = &ctx.accounts.treasury_lp.to_account_info();

     let treasury_token = &ctx.accounts.treasury_token.to_account_info();
     let token_vault = &ctx.accounts.token_vault.to_account_info();
     let token_program = &ctx.accounts.token_program.to_account_info();
     let vault_program = &ctx.accounts.vault_program.to_account_info();
     let user = &&ctx.accounts.treasury_authority.to_account_info();
     update_liquidity_wrapper(
         move || {
             MeteoraUtils::withdraw(
                 vault,
                 vault_lp_mint,
                 treasury_token,
                 treasury_lp,
                 user,  // there is difference from deposit
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
         &mut ctx.accounts.treasury_lp,
         &mut ctx.accounts.partner,
        //  &mut ctx.accounts.user,
     )?;
     Ok(())
 }