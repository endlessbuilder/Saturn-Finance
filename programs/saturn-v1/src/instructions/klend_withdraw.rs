use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use kamino_lending::{
    cpi::accounts::WithdrawObligationCollateralAndRedeemReserveCollateral, program::KaminoLending,
    Obligation,
};
use crate::{
    account::{Escrow, Treasury},
    constants::*,
};
use anchor_lang::solana_program::sysvar;

#[derive(Accounts)]
pub struct KlendWithdraw<'info> {
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

    #[account(mut)]
    pub user_destination_liquidity: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    /// CHECK: address on account checked
    #[account(address = sysvar::instructions::ID)]
    pub instructions: AccountInfo<'info>,

    /*
     * klend accounts
     */
    pub klend_program: Program<'info, KaminoLending>,
    /// CHECK: devnet demo
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
    pub reserve_source_deposit_collateral: AccountInfo<'info>,
    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: devnet demo
    pub user_destination_collateral: Option<AccountInfo<'info>>,
}

pub fn handle(ctx: Context<KlendWithdraw>, amount: u64) -> Result<()> {
    // let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
       TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    kamino_lending::cpi::withdraw_obligation_collateral_and_redeem_reserve_collateral(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            WithdrawObligationCollateralAndRedeemReserveCollateral {
                owner: ctx.accounts.saturn_lending.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
                withdraw_reserve: ctx.accounts.reserve.to_account_info(),
                reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
                reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.to_account_info(),
                user_destination_collateral: ctx
                    .accounts
                    .user_destination_collateral
                    .to_account_info(),
                // placeholder_user_destination_collateral: Some(ctx.accounts.user_destination_collateral.clone().expect("REASON").to_account_info()),
                token_program: ctx.accounts.token_program.to_account_info(),
                instruction_sysvar_account: ctx.accounts.instructions.to_account_info(),
                user_destination_liquidity: ctx
                    .accounts
                    .user_destination_liquidity
                    .to_account_info(),
                reserve_source_collateral: ctx
                    .accounts
                    .reserve_source_deposit_collateral
                    .to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )?;

    let treasury = &mut ctx.accounts.treasury;
    treasury.kamino_lend_amount -= amount;
    treasury.treasury_value += amount;
    
    Ok(())    
}


