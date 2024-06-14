use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use marginfi::{
    cpi::accounts::LendingAccountDeposit, program::Marginfi,
    state::marginfi_group::{MarginfiGroup, Bank},
};
use crate::{
    account::{Escrow, Treasury},
    constants::*,
};


#[derive(Accounts)]
pub struct MarginfiLend<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global-treasury", saturn_lending.treasury_admin.key().as_ref()],
        bump,
    )]
    pub saturn_lending: Account<'info, Treasury>,

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

    #[account(
        mut
    )]
    pub saturn_liquidity: Account<'info, TokenAccount>,

    /// CHECK: marginfi account
    #[account(mut)]
    pub bank_liquidity_vault: AccountInfo<'info>,
}

pub fn handle(ctx: Context<MarginfiLend>, amount: u64) -> Result<()> {
    let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        b"global-treasury",
        owner_key.as_ref(),
        &[ctx.bumps.saturn_lending],
    ];

    marginfi::cpi::lending_account_deposit(
        CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info(),
            LendingAccountDeposit {
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                signer: ctx.accounts.saturn_lending.to_account_info(),
                bank: ctx.accounts.bank.to_account_info(),
                signer_token_account: ctx
                    .accounts
                    .saturn_liquidity
                    .to_account_info(),
                bank_liquidity_vault: ctx.accounts.bank_liquidity_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )

}

