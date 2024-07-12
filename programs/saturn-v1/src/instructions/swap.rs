use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, instruction::Instruction, program::invoke_signed},
    system_program,
};
use crate::{
    account::{Escrow, Treasury, UserStakeAccount},
    constants::*,
    error::*,
    jupiter_utils::*
};
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};


#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    /// CHECK: This may not be initialized yet.
    #[account(mut, seeds = [from_mint.key().as_ref()], bump)]
    pub from_treasury_token_account: UncheckedAccount<'info>,

    pub from_mint: Account<'info, Mint>,

    /// CHECK: This may not be initialized yet.
    #[account(mut, seeds = [to_mint.key().as_ref()], bump)]
    pub to_treasury_token_account: UncheckedAccount<'info>,

    pub to_mint: Account<'info, Mint>,

    pub jupiter_program: Program<'info, Jupiter>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Swap>, data: Vec<u8>, from_amount: u64) -> Result<()> {

    let authority_bump = ctx.bumps.treasury_authority;
    let to_treasury_token_account_bump = ctx.bumps.to_treasury_token_account;
    let from_treasury_token_account_bump = ctx.bumps.from_treasury_token_account;
    let wsol_mint = Pubkey::from_str(WSOL_MINT).unwrap();
    // let from_amount = 1000000;
    

    if ctx.accounts.from_mint.key() == wsol_mint {
        create_wsol_token_idempotent(
            ctx.accounts.treasury_authority.clone(),
            ctx.accounts.from_treasury_token_account.clone(),
            ctx.accounts.from_mint.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
            authority_bump,
            from_treasury_token_account_bump,
            from_amount
        )?;
    }

    if ctx.accounts.to_mint.key() == wsol_mint {
        create_wsol_token_idempotent(
            ctx.accounts.treasury_authority.clone(),
            ctx.accounts.to_treasury_token_account.clone(),
            ctx.accounts.to_mint.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
            authority_bump,
            to_treasury_token_account_bump,
            0u64
        )?;
    }
    msg!("Swap on Jupiter");
    
    let authority_bump_seeds = [authority_bump];
    let signer_seeds: &[&[&[u8]]] = &[&[TREASURY_AUTHORITY_SEED.as_bytes(), authority_bump_seeds.as_ref()]];
    swap_on_jupiter(
        ctx.remaining_accounts,
        ctx.accounts.jupiter_program.clone(),
        data,
        signer_seeds,
        ctx.accounts.treasury_authority.key
    )?;

    if ctx.accounts.from_mint.key() == wsol_mint {
        close_treasury_wsol(
            ctx.accounts.treasury_authority.clone(),
            ctx.accounts.from_treasury_token_account.clone(),
            ctx.accounts.token_program.clone(),
            &[authority_bump],
        )?;
    }

    if ctx.accounts.to_mint.key() == wsol_mint {
        close_treasury_wsol(
            ctx.accounts.treasury_authority.clone(),
            ctx.accounts.to_treasury_token_account.clone(),
            ctx.accounts.token_program.clone(),
            &[authority_bump],
        )?;
    }

    Ok(())
}

