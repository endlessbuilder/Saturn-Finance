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
    utils::*
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
    #[account(mut, seeds = [to_mint.key().as_ref()], bump)]
    pub treasury_token_account: UncheckedAccount<'info>,

    pub to_mint: Account<'info, Mint>,

    pub jupiter_program: Program<'info, Jupiter>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Swap>, data: Vec<u8>) -> Result<()> {

    let authority_bump = ctx.bumps.treasury_authority;
    let wsol_bump = ctx.bumps.treasury_token_account;
    let wsol_mint = Pubkey::from_str(WSOL_MINT).unwrap();

    if ctx.accounts.to_mint.key() == wsol_mint {
        create_wsol_token_idempotent(
            ctx.accounts.treasury_authority.clone(),
            ctx.accounts.treasury_token_account.clone(),
            ctx.accounts.to_mint.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
            authority_bump,
            wsol_bump,
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

    if ctx.accounts.to_mint.key() == wsol_mint {
        close_treasury_wsol(
            ctx.accounts.treasury_authority.clone(),
            ctx.accounts.treasury_token_account.clone(),
            ctx.accounts.token_program.clone(),
            &[authority_bump],
        )?;
    }

    Ok(())
}

