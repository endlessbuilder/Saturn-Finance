use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, instruction::Instruction, program::invoke_signed},
    system_program,
};
use anchor_spl::token::{Mint, Token, TokenAccount};
use meteora::{get_base_key, state::Vault};
use std::convert::TryFrom;
use std::str::FromStr;

use crate::account::meteora_account::Partner;
use crate::constants::DEFAULT_FEE_RATIO;
use crate::meteora_utils::get_admin_address;
use crate::error::VaultError;

/// MeteoraInitialize struct
#[derive(Accounts)]
pub struct MeteoraInitialize<'info> {
    /// Vault account
    #[account(
            init,
            seeds = [
                vault.key().as_ref(), partner_token.key().as_ref(),
            ],
            bump,
            payer = admin,
            space = 200 // data + buffer,
        )]
    pub partner: Box<Account<'info, Partner>>,
    /// CHECK: vault account that partner integrates to
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK: partner token account that is integrated to dynamic vault  partner_token mint must be same as native token in vault
    #[account(constraint = vault.token_mint == partner_token.mint)]
    pub partner_token: Box<Account<'info, TokenAccount>>,

    /// dynamic vault Admin address
    #[account(mut, constraint = admin.key() == get_admin_address())]
    pub admin: Signer<'info>,

    /// System program account
    pub system_program: Program<'info, System>,
    /// Rent account
    pub rent: Sysvar<'info, Rent>,
    /// Token program account
    pub token_program: Program<'info, Token>,
}

#[allow(unused_variables)]
pub fn handle(ctx: Context<MeteoraInitialize>) -> Result<()> {
    let partner = &mut ctx.accounts.partner;
    partner.vault = ctx.accounts.vault.key();
    partner.partner_token = ctx.accounts.partner_token.key();
    partner.fee_ratio = DEFAULT_FEE_RATIO;
    partner.bump = ctx.bumps.partner;
    Ok(())
}
