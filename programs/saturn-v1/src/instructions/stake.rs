use anchor_lang::prelude::*;
use crate::{
    account::{Escrow, Treasury, UserStakeAccount},
    constants::*,
    error::*,
    utils::*
};


#[derive(Accounts)]
#[instruction(args: ApplyBondArgs)]
pub struct StakeSTF<'info> {
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
        constraint = creater_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = creater_token_account.owner == *user.key,
    )]
    pub user_account_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = dest_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = dest_token_account.owner == *treasury.to_account_info().key,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stf_token_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<StakeSTF>, amount_to_stake: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let source_token_account = &mut &ctx.accounts.user_account_token;
    let dest_stf_account = &mut &ctx.accounts.treasury_token_account;
    let stf_token_mint = &mut &ctx.accounts.stf_token_mint;
    let user = &mut ctx.account.user;
    let personal_account = &mut ctx.accounts.user_program_account;

    assert!(
        stf_token_mint.key().to_string().as_str() == STF_MINT,
        "STF_TOKEN_MINT ERROR"
    );


    // Transfer Tokens To Treasury 
    let cpi_accounts = SplTransfer {
        from: source_token_account.to_account_info().clone(),
        to: dest_stf_account.to_account_info().clone(),
        authority: user.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();
    
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
    amount_to_stake)?;

    // Add STF
    let amount_to_transfer = amount_to_stake / treasury.staking_index;
    personal_account.total_staked_index  += amount_to_transfer;




    Ok(())
}