use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::account::meteora_account::{Partner, User};
use crate::constants::{TOKEN_VAULT_PREFIX, VAULT_PREFIX};
// use crate::meteora_context::DepositWithdrawLiquidity;
use crate::meteora_utils::{MeteoraProgram, MeteoraUtils, update_liquidity_wrapper};
use meteora::state::Vault;

/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct MeteoraDeposit<'info> {
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

#[allow(unused_variables)]
pub fn handle(
    ctx: Context<MeteoraDeposit>,
    token_amount: u64,
    minimum_lp_token_amount: u64,
) -> Result<()> {
    let vault = &ctx.accounts.vault.to_account_info();
    let vault_lp_mint = &ctx.accounts.vault_lp_mint.to_account_info();
    let user_lp = &ctx.accounts.user_lp.to_account_info();

    let user_token = &ctx.accounts.user_token.to_account_info();
    let token_vault = &ctx.accounts.token_vault.to_account_info();
    let token_program = &ctx.accounts.token_program.to_account_info();
    let vault_program = &ctx.accounts.vault_program.to_account_info();
    let owner = &ctx.accounts.owner.to_account_info();

    update_liquidity_wrapper(
        move || {
            MeteoraUtils::deposit(
                vault,
                vault_lp_mint,
                user_token,
                user_lp, // mint vault lp token to pool lp token account
                owner,
                token_vault,
                token_program,
                vault_program,
                token_amount,
                minimum_lp_token_amount,
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
