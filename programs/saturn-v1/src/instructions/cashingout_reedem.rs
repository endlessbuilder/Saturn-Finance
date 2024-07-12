use anchor_lang::prelude::*;
use anchor_spl::token::{self, mint_to, Mint, MintTo, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::pubkey::Pubkey;

use crate::{constants::*, account::{Escrow, Treasury}};
use crate::error::BondError;

#[derive(Accounts)]
pub struct CashingoutReedem<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury_authority.key()
    )]
    signer: Signer<'info>,
    /// CHECK: 
    #[account(mut)]
    pub user: UncheckedAccount<'info>,

    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [ESCROW.as_ref(), user.key.as_ref()],
        bump,
    )]
    pub escrow: AccountLoader<'info, Escrow>,

    #[account(
        mut,
        constraint = user_token_account.mint.key().to_string().as_str() == USDC_MINT,
        constraint = user_token_account.owner == *user.to_account_info().key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.key().to_string().as_str() == USDC_MINT,
        constraint = treasury_token_account.owner == *treasury_authority.to_account_info().key,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = fee_wallet_token_account.key().to_string().as_str() == USDC_MINT,
    )]
    pub fee_wallet_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = treasury_stf_token_account.mint == *stf_token_mint.to_account_info().key,
        constraint = treasury_stf_token_account.owner == *treasury_authority.to_account_info().key,
    )]
    pub treasury_stf_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_mint_address: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stf_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<CashingoutReedem>,
    amount: u64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow.load_mut().unwrap();
    let user = &mut &ctx.accounts.user;
    let treasury = &mut ctx.accounts.treasury;
    let treasury_authority = &mut ctx.accounts.treasury_authority;
    let user_token_account = &mut &ctx.accounts.user_token_account;
    let treasury_token_account = &mut &ctx.accounts.treasury_token_account;
    let fee_wallet = &mut &ctx.accounts.fee_wallet_token_account;
    let treasury_stf_token_account = &mut &ctx.accounts.treasury_stf_token_account;
    let token_program = &mut &ctx.accounts.token_program;
    let stf_token_mint = &mut &ctx.accounts.stf_token_mint;

    if amount < escrow.num_token_to_redeem {
        return Err(BondError::InsufficientFundsError.into());
    }

    let backing_price = treasury.treasury_value / treasury.token_minted * 100;
    let cashingout_value = amount * backing_price;

    let reedem = ( cashingout_value as f64 * 0.99 ) as u64;
    let fee = ( cashingout_value as f64 * 0.05 ) as u64;

    let cpi_accounts = Transfer {
        from: treasury_token_account.to_account_info().clone(),
        to: user_token_account.to_account_info().clone(),
        authority: ctx.accounts.treasury_authority.to_account_info().clone(),
    };
    token::transfer(
        CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
        reedem
    )?;

    let cpi_accounts = Transfer {
        from: treasury_token_account.to_account_info().clone(),
        to: fee_wallet.to_account_info().clone(),
        authority: ctx.accounts.treasury_authority.to_account_info().clone(),
    };
    token::transfer(
        CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
        fee
    )?;

    treasury.token_minted -= amount;
    treasury.treasury_value = treasury.treasury_value - reedem - fee;
    

    Ok(())
}