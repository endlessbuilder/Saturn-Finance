use crate::{
    account::{Treasury, SequenceFlag},
    constants::*,
    treasury,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use marginfi::{
    cpi::accounts::LendingAccountDeposit,
    program::Marginfi,
    state::marginfi_group::{Bank, MarginfiGroup},
    state::marginfi_account::MarginfiAccount,
};

#[derive(Accounts)]
pub struct MarginfiLend<'info> {
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

    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [SEQUENCE_FLAG_SEED.as_ref()],
        bump,
        constraint = sequence_flag.flag_calcu_balance == true,
        constraint = sequence_flag.flag_reallocate == true,
        constraint = sequence_flag.flag_kamino && sequence_flag.flag_marginfi && sequence_flag.flag_meteora  == true,
    )]
    pub sequence_flag: Account<'info, SequenceFlag>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

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
    pub bank_liquidity_vault: AccountInfo<'info>,
}

pub fn handle(ctx: Context<MarginfiLend>) -> Result<()> {
    let treasury = &ctx.accounts.treasury;
    let amount = (treasury.marginfi_allocation * treasury.treasury_value) * (1_000_000) as f64;
    // let owner_key = ctx.accounts.treasury_authority.to_account_info();
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    let _ = marginfi::cpi::lending_account_deposit(
        CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info(),
            LendingAccountDeposit {
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                signer: ctx.accounts.treasury_authority.to_account_info(),
                bank: ctx.accounts.bank.to_account_info(),
                signer_token_account: ctx.accounts.user_liquidity.to_account_info(),
                bank_liquidity_vault: ctx.accounts.bank_liquidity_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount as u64,
    );

    Ok(())
}
