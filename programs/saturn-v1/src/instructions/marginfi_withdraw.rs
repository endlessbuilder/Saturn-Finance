use crate::{
    account::{Treasury, SequenceFlag},
    constants::*,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use marginfi::{
    cpi::accounts::LendingAccountWithdraw,
    program::Marginfi,
    state::marginfi_account::MarginfiAccount,
    state::marginfi_group::{Bank, MarginfiGroup},
};

#[derive(Accounts)]
pub struct MarginfiWithdraw<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury_authority.key()
    )]
    signer: Signer<'info>,
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

    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [SEQUENCE_FLAG_SEED.as_ref()],
        bump,
        constraint = sequence_flag.flag_calcu_balance == true,
        constraint = sequence_flag.flag_reallocate == true,
        constraint = sequence_flag.flag_marginfi  == false,
    )]
    pub sequence_flag: Account<'info, SequenceFlag>,

    /*
     * marginfi accounts
     */
    pub marginfi_program: Program<'info, Marginfi>,

    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,

    #[account(
        mut,
        constraint = marginfi_account.load()?.group == marginfi_group.key(),
        constraint = marginfi_account.load()?.authority == treasury_authority.key(),
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    #[account(
        mut,
        constraint = bank.load()?.group == marginfi_group.key(),
    )]
    pub bank: AccountLoader<'info, Bank>,

    #[account(mut)]
    pub user_liquidity: Account<'info, TokenAccount>,

    /// CHECK: marginfi account
    #[account(mut)]
    pub bank_liquidity_vault: Account<'info, TokenAccount>,
    /// CHECK: Seed constraint check
    #[account(mut)]
    pub bank_liquidity_vault_authority: AccountInfo<'info>,
}

pub fn handle(ctx: Context<MarginfiWithdraw>) -> Result<()> {
    let amount = ctx.accounts.treasury.marginfi_lend_assets;
    // let owner_key = ctx.accounts.treasury_authority;
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    let _ = marginfi::cpi::lending_account_withdraw(
        CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info(),
            LendingAccountWithdraw {
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                signer: ctx.accounts.treasury_authority.to_account_info(),
                bank: ctx.accounts.bank.to_account_info(),
                destination_token_account: ctx.accounts.user_liquidity.to_account_info(),
                bank_liquidity_vault_authority: ctx.accounts.bank_liquidity_vault_authority.to_account_info(),
                bank_liquidity_vault: ctx.accounts.bank_liquidity_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[signer_seeds], //remaining_accounts: ctx.remaining_accounts.into(),
        ),
        amount,
        None,
    );

    ctx.accounts.sequence_flag.flag_marginfi = true;

    Ok(())
}
