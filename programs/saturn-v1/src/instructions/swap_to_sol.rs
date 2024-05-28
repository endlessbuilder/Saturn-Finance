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
    #[account(mut, seeds = [AUTHORITY_SEED], bump)]
    pub program_authority: SystemAccount<'info>,
    /// CHECK: This may not be initialized yet.
    #[account(mut, seeds = [WSOL_SEED], bump)]
    pub program_wsol_account: UncheckedAccount<'info>,
    pub user_account: Signer<'info>,
    #[account(address = spl_token::native_mint::id())]
    pub sol_mint: Account<'info, Mint>,
    pub jupiter_program: Program<'info, Jupiter>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<SwapToSOL>, data: Vec<u8>) -> Result<()> {

    let authority_bump = ctx.bumps.program_authority;
    let wsol_bump = ctx.bumps.program_wsol_account;

    create_wsol_token_idempotent(
        ctx.accounts.program_authority.clone(),
        ctx.accounts.program_wsol_account.clone(),
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

    let after_swap_lamports = ctx.accounts.program_wsol_account.lamports();

    close_program_wsol(
        ctx.accounts.program_authority.clone(),
        ctx.accounts.program_wsol_account.clone(),
        ctx.accounts.token_program.clone(),
        &[authority_bump],
    )?;

    let rent = Rent::get()?;
    let space = TokenAccount::LEN;
    let token_lamports = rent.minimum_balance(space);
    let out_amount = after_swap_lamports - token_lamports;

    msg!("Transfer SOL to user");
    let signer_seeds: &[&[&[u8]]] = &[&[AUTHORITY_SEED, &[authority_bump]]];
    let lamports = out_amount;
    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.program_authority.to_account_info(),
                to: ctx.accounts.user_account.to_account_info(),
            },
            signer_seeds,
        ),
        lamports,
    )?;

    Ok(())
}

