use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::account::Treasury;
use crate::account::meteora_account::Partner;
use crate::constants::{TOKEN_VAULT_PREFIX, TREASURY_AUTHORITY_SEED, VAULT_PREFIX, TREASURY_SEED};
// use crate::meteora_context::DepositWithdrawLiquidity;
use crate::meteora_utils::{MeteoraProgram, MeteoraUtils, update_liquidity_wrapper};
use meteora::state::Vault;

/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct MeteoraDeposit<'info> {
    /// partner info CHECK:
    #[account(mut, has_one = vault)]
    pub partner: Box<Account<'info, Partner>>,
    /// user CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
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

#[allow(unused_variables)]
pub fn handle(
    ctx: Context<MeteoraDeposit>,
    token_amount: u64,
    minimum_lp_token_amount: u64,
) -> Result<()> {
    let vault = &ctx.accounts.vault.to_account_info();
    let vault_lp_mint = &ctx.accounts.vault_lp_mint.to_account_info();
    let treasury_lp = &ctx.accounts.treasury_lp.to_account_info();

    let treasury_token = &ctx.accounts.treasury_token.to_account_info();
    let token_vault = &ctx.accounts.token_vault.to_account_info();
    let token_program = &ctx.accounts.token_program.to_account_info();
    let vault_program = &ctx.accounts.vault_program.to_account_info();
    let user = &&ctx.accounts.treasury_authority.to_account_info();

    update_liquidity_wrapper(
        move || {
            MeteoraUtils::deposit(
                vault,
                vault_lp_mint,
                treasury_token,
                treasury_lp, // mint vault lp token to pool lp token account
                user,
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
        &mut ctx.accounts.treasury_lp,
        &mut ctx.accounts.partner,
        // &mut ctx.accounts.user,
    )?;

    let treasury = &mut ctx.accounts.treasury;
    treasury.meteora_deposit_amount += token_amount;

    Ok(())
}
