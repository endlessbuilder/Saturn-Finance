use anchor_lang::prelude::*;
use context::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::convert::TryFrom;
use std::str::FromStr;

use crate::account::meteora_account::*;
use crate::meteora_utils::*;
use crate::constants::{VAULT_PREFIX, TOKEN_VAULT_PREFIX, LP_MINT_PREFIX};

/// Accounts for initialize a new vault
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// This is base account for all vault    
    /// No need base key now because we only allow 1 vault per token now
    // pub base: Signer<'info>,

    /// Vault account
    #[account(
        init,
        seeds = [
            seed::VAULT_PREFIX.as_ref(), token_mint.key().as_ref(), get_base_key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Vault>(),
    )]
    pub vault: Box<Account<'info, Vault>>,

    /// Payer can be anyone
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Token vault account
    #[account(
        init,
        seeds = [seed::TOKEN_VAULT_PREFIX.as_ref(), vault.key().as_ref()],
        bump,
        payer = payer,
        token::mint = token_mint,
        token::authority = vault,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,
    /// Token mint account
    pub token_mint: Box<Account<'info, Mint>>, // allocate some accounts in heap to avoid stack frame size limit
    #[account(
        init,
        seeds = [seed::LP_MINT_PREFIX.as_ref(), vault.key().as_ref()],
        bump,
        payer = payer,
        mint::decimals = token_mint.decimals,
        mint::authority = vault,
    )]
    pub lp_mint: Box<Account<'info, Mint>>,
    /// rent
    pub rent: Sysvar<'info, Rent>,
    /// token_program
    pub token_program: Program<'info, Token>,
    /// system_program
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}