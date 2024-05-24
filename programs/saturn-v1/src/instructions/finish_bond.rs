use anchor_lang::prelude::*;
use anchor_spl::token::{self, mint_to, MintTo, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    account::{Escrow, Treasury},
    constants::*,
    error::BondError,
};

#[derive(Accounts)]
#[instruction()]
pub struct FinishBond<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        constraint = dest_stf_account.mint == *stf_token_mint.to_account_info().key,
        constraint = dest_stf_account.owner == *admin.to_account_info().key,
    )]
    pub dest_stf_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow: AccountLoader<'info, Escrow>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stf_token_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<FinishBond>, global_bump: u8) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    let escrow = &mut ctx.accounts.escrow.load_mut()?;
    let treasury = &mut ctx.accounts.treasury;
    let stf_token_mint = &mut &ctx.accounts.stf_token_mint;
    let dest_stf_account = &mut &ctx.accounts.dest_stf_account;
    let token_program = &mut &ctx.accounts.token_program;

    // if timestamp < escrow.end_timestamp {
    //     return Err(BondError::BondNotFinished.into());
    // }
    if escrow.creator != ctx.accounts.admin.key() {
        return Err(BondError::CreatorError.into());
    }
    if escrow.is_finished != 1 {
        return Err(BondError::AlreadyRedeem.into());
    }

    //Mint Token to Redeem to the creator
    let seeds = &[TREASURY_SEED.as_bytes(), &[global_bump]];
    let signer = &[&seeds[..]];
    
    let cpi_accounts = MintTo {
        mint: stf_token_mint.to_account_info().clone(),
        to: dest_stf_account.to_account_info().clone(),
        authority: ctx.accounts.treasury.to_account_info().clone(),
    };
    token::mint_to(
        CpiContext::new_with_signer(
            token_program.clone().to_account_info(),
            cpi_accounts,
            signer,
        ),
        escrow.num_token_to_redeem,
    )?;
    escrow.is_finished = 2;    
    Ok(())
}
