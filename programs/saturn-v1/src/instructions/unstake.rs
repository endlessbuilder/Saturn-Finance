use std::mem;

use anchor_lang::prelude::*;
use crate::{
    account::{Escrow, Treasury, UserStakeAccount},
    constants::*,
    error::*,
    utils::*
};
use anchor_spl::{token::{Mint, Token, TokenAccount}, token_interface::{transfer_checked, TransferChecked}};


#[derive(Accounts)]

pub struct UnStakeSTF<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        space = mem::size_of::<UserStakeAccount>() as usize + 8,
        payer = user,
        seeds=[PERSONAL_SEED.as_ref(), user.key.as_ref()], // Static Seed Path (1)
        bump, 
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,
    
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
        constraint = user_token_account.mint == *stf_token_mint.to_account_info().key,
        constraint = user_token_account.owner == *user.to_account_info().key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == *stf_token_mint.to_account_info().key,
        constraint = treasury_token_account.owner == *treasury_authority.to_account_info().key,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stf_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


// Amount to unstake is in sSTF
pub fn handle(ctx: Context<UnStakeSTF>, amount_to_unstake: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let treasury_authority = &mut ctx.accounts.treasury_authority;
    let user_token_account = &mut &ctx.accounts.user_token_account;
    let treasury_token_account = &mut &ctx.accounts.treasury_token_account;
    let stf_token_mint = &mut &ctx.accounts.stf_token_mint;
    let user = &mut ctx.accounts.user;
    let personal_account = &mut ctx.accounts.user_stake_account;
    let authority_bump = ctx.bumps.treasury_authority;


    require!(personal_account.total_staked_index as u64 > amount_to_unstake, BondError::UnstakingError); 

    // assert!(
    //     stf_token_mint.key().to_string().as_str() == STF_MINT,
    //     "STF_TOKEN_MINT ERROR"
    // );
    // Add STF
    let amount_to_transfer = amount_to_unstake * treasury.staking_index;
    personal_account.total_staked_index  -= amount_to_unstake;
    treasury.token_staked -= amount_to_unstake;

    let accounts = TransferChecked {
        from: treasury_token_account.to_account_info(),
        to: user_token_account.to_account_info(),
        authority: treasury_authority.to_account_info(),
        mint: stf_token_mint.to_account_info()
    };

    let authority_bump_seeds = [authority_bump];
    let signer_seeds: &[&[&[u8]]] = &[&[TREASURY_AUTHORITY_SEED.as_bytes(), authority_bump_seeds.as_ref()]];

    let ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds
    );

    let _ = transfer_checked(ctx, amount_to_transfer, 2);



    Ok(())
}