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
pub struct SwapToSOL<'info> {
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    /// CHECK: This may not be initialized yet.
    #[account(mut, seeds = [WSOL_SEED], bump)]
    pub treasury_wsol_account: UncheckedAccount<'info>,
    #[account(address = spl_token::native_mint::id())]
    pub sol_mint: Account<'info, Mint>,
    pub jupiter_program: Program<'info, Jupiter>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<SwapToSOL>, data: Vec<u8>) -> Result<()> {

    let authority_bump = ctx.bumps.treasury_authority;
    let wsol_bump = ctx.bumps.treasury_wsol_account;

    create_wsol_token_idempotent(
        ctx.accounts.treasury_authority.clone(),
        ctx.accounts.treasury_wsol_account.clone(),
        ctx.accounts.sol_mint.clone(),
        ctx.accounts.token_program.clone(),
        ctx.accounts.system_program.clone(),
        &[authority_bump],
        &[wsol_bump],
    )?;

    msg!("Swap on Jupiter");
    swap_on_jupiter(
        ctx.remaining_accounts,
        ctx.accounts.jupiter_program.clone(),
        data,
    )?;

    let after_swap_lamports = ctx.accounts.treasury_wsol_account.lamports();

    close_treasury_wsol(
        ctx.accounts.treasury_authority.clone(),
        ctx.accounts.treasury_wsol_account.clone(),
        ctx.accounts.token_program.clone(),
        &[authority_bump],
    )?;



    Ok(())
}

