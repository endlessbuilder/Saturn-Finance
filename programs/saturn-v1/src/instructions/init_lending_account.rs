use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use marginfi::{
    cpi::accounts::MarginfiAccountInitialize, program::Marginfi,
    state::marginfi_group::MarginfiGroup,
};
use kamino_lending::{
    cpi::accounts::{InitObligation, InitUserMetadata},
    program::KaminoLending,
    InitObligationArgs,
};
use crate::{
    account::{Escrow, Treasury},
    constants::*,
};

pub fn handle(ctx: Context<InitLendingAccount>) -> Result<()> {
    let owner_key = ctx.accounts.treasury_authority.key();
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];


    // init klend account and create obligation
    kamino_lending::cpi::init_user_metadata(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            InitUserMetadata {
                owner: ctx.accounts.treasury_authority.to_account_info(),
                fee_payer: ctx.accounts.owner.to_account_info(),
                user_metadata: ctx.accounts.user_metadata.to_account_info(),
                referrer_user_metadata: Option::None,
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        Pubkey::default(), // user_lookup_table
    )?;

    kamino_lending::cpi::init_obligation(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            InitObligation {
                obligation_owner: ctx.accounts.treasury_authority.to_account_info(),
                fee_payer: ctx.accounts.owner.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                seed1_account: ctx.accounts.seed1_account.to_account_info(),
                seed2_account: ctx.accounts.seed2_account.to_account_info(),
                owner_user_metadata: ctx.accounts.user_metadata.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        InitObligationArgs { tag: 0, id: 0 },
    )?;

     // init marginfi account
     marginfi::cpi::marginfi_account_initialize(CpiContext::new_with_signer(
        ctx.accounts.marginfi_program.to_account_info(),
        MarginfiAccountInitialize {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
            fee_payer: ctx.accounts.owner.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[signer_seeds],
    ))?;
    Ok(())
}

#[derive(Accounts)]
pub struct InitLendingAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    
    /*
    * marginfi accounts
    */
    pub marginfi_program: Program<'info, Marginfi>,
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,
        
    /*
    * klend accounts
    */
    /// CHECK: devnet demo
    pub klend_program: Program<'info, KaminoLending>,
    /// CHECK: devnet demo
    pub seed1_account: AccountInfo<'info>,
    /// CHECK: devnet demo
    pub seed2_account: AccountInfo<'info>,
    /// CHECK: devnet demo
    pub lending_market: AccountInfo<'info>,
    /// CHECK: devnet demo
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    /// CHECK: devnet demo
    #[account(mut)]
    pub user_metadata: AccountInfo<'info>,
    
    #[account(
        mut,
    )]
    /// CHECK: passed to marginfi
    pub marginfi_account: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
