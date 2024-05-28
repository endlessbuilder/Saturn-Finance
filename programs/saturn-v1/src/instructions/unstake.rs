use std::mem;

use anchor_lang::prelude::*;
use crate::{
    account::{Escrow, Treasury, UserStakeAccount},
    constants::*,
    error::*,
    utils::*
};
use anchor_spl::{token::{Token, TokenAccount}, token_interface::{transfer_checked, TransferChecked}};


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
    pub user_program_account: Account<'info, UserStakeAccount>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(
        mut,
        constraint = user_account_token.mint == *stf_token_mint.to_account_info().key,
        constraint = user_account_token.owner == *user.key,
    )]
    pub user_account_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == *stf_token_mint.to_account_info().key,
        constraint = treasury_token_account.owner == *treasury.to_account_info().key,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stf_token_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


// Amount to unstake is in sSTF
pub fn handle(ctx: Context<UnStakeSTF>, amount_to_unstake: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let user_token_account = &mut &ctx.accounts.user_account_token;
    let treasury_token_account = &mut &ctx.accounts.treasury_token_account;
    let stf_token_mint = &mut &ctx.accounts.stf_token_mint;
    let user = &mut ctx.accounts.user;
    let personal_account = &mut ctx.accounts.user_program_account;


    require!(personal_account.total_staked_index as u64 > amount_to_unstake, BondError::UnstakingError); 

    assert!(
        stf_token_mint.key().to_string().as_str() == STF_MINT,
        "STF_TOKEN_MINT ERROR"
    );
    // Add STF
    let amount_to_transfer = amount_to_unstake * treasury.staking_index;
    personal_account.total_staked_index  -= amount_to_unstake;
    treasury.token_staked -= amount_to_unstake;

    let accounts = TransferChecked {
        from: treasury_token_account.to_account_info(),
        to: user_token_account.to_account_info(),
        authority: treasury_token_account.to_account_info(),
        mint: stf_token_mint.to_account_info()
    };

    let seeds = &[
        &b"TREASURY_SEED"[..],
    ];

    let signer_seeds = &[&seeds[..]];

    let ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds
    );

    let _ = transfer_checked(ctx, amount_to_transfer, 9);



    Ok(())
}