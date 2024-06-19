use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use kamino_lending::{
    cpi::accounts::DepositReserveLiquidityAndObligationCollateral, program::KaminoLending,
    Obligation,
};
use crate::{
    account::{Escrow, Treasury},
    constants::*, treasury,
};


#[derive(Accounts)]
pub struct KaminoLend<'info> {
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

    #[account(
        mut,        
    )]
    pub saturn_user_source_liquidity: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    /*
     * klend accounts
     */
    pub klend_program: Program<'info, KaminoLending>,
    #[account(mut)]
    pub obligation: AccountLoader<'info, Obligation>,
    /// CHECK: devnet demo
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: devnet demo
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: devnet demo
    pub reserve_liquidity_supply: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: devnet demo
    pub reserve_collateral_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: devnet demo
    pub reserve_destination_deposit_collateral: AccountInfo<'info>,
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: devnet demo
    pub user_destination_collateral: Option<AccountInfo<'info>>,
    /// CHECK: devnet demo
    pub instruction_sysvar_account: AccountInfo<'info>,
}


pub fn handle(ctx: Context<KaminoLend>, amount: u64) -> Result<()> {
    // let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    kamino_lending::cpi::deposit_reserve_liquidity_and_obligation_collateral(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            DepositReserveLiquidityAndObligationCollateral {
                owner: ctx.accounts.treasury_authority.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
                reserve: ctx.accounts.reserve.to_account_info(),
                reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
                reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.to_account_info(),
                reserve_destination_deposit_collateral: ctx
                    .accounts
                    .reserve_destination_deposit_collateral
                    .to_account_info(),
                user_source_liquidity: ctx
                    .accounts
                    .saturn_user_source_liquidity
                    .to_account_info(),
                user_destination_collateral: ctx
                    .accounts
                    .user_destination_collateral
                    .to_account_info(),
                // placeholder_user_destination_collateral: Some(ctx.accounts.user_destination_collateral.clone().expect("REASON").to_account_info()),
                token_program: ctx.accounts.token_program.to_account_info(),
                instruction_sysvar_account: ctx
                    .accounts
                    .instruction_sysvar_account
                    .to_account_info(),
                user_destination_collateral: todo!(),
            },
            &[signer_seeds],
        ),
        amount,
    )?;

    let treasury = &mut ctx.accounts.treasury;
    treasury.kamino_lend_amount += amount;
    treasury.treasury_value -= amount;

    Ok(())

}

