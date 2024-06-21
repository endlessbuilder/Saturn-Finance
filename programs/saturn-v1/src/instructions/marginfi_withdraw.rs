use crate::{
    account::{Escrow, Treasury},
    constants::*,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use marginfi::{
    cpi::accounts::LendingAccountWithdraw,
    program::Marginfi,
    state::marginfi_group::{Bank, MarginfiGroup},
};

#[derive(Accounts)]
pub struct MarginfiWithdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK:
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

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

    /*
     * marginfi accounts
     */
    pub marginfi_program: Program<'info, Marginfi>,
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,
    #[account(mut)]
    /// CHECK: passed to marginfi
    pub marginfi_account: Signer<'info>,

    #[account(
        mut,
        constraint = bank.load()?.group == marginfi_group.key(),
    )]
    pub bank: AccountLoader<'info, Bank>,

    #[account(mut)]
    pub saturn_liquidity: Account<'info, TokenAccount>,

    /// CHECK: marginfi account
    #[account(mut)]
    pub bank_liquidity_vault: AccountInfo<'info>,
}

pub fn handle(ctx: Context<MarginfiWithdraw>, amount: u64) -> Result<()> {
    let owner_key = ctx.accounts.treasury.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    marginfi::cpi::lending_account_withdraw(
        CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info(),
            LendingAccountWithdraw {
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                signer: ctx.accounts.treasury_authority.to_account_info(),
                bank: ctx.accounts.bank.to_account_info(),
                destination_token_account: ctx.accounts.saturn_liquidity.to_account_info(),
                bank_liquidity_vault_authority: ctx.accounts.bank_liquidity_vault.to_account_info(),
                bank_liquidity_vault: ctx.accounts.bank_liquidity_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[signer_seeds], //remaining_accounts: ctx.remaining_accounts.into(),
        ),
        amount,
        None,
    )

    let treasury = &mut ctx.accounts.treasury;
    treasury.marginfi_lend_amount -= amount;
    treasury.treasury_value += amount;

    Ok(())
}