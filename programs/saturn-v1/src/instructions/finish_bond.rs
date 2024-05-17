use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{constants::*, account::{Escrow, Treasury}};

#[derive(Accounts)]
#[instruction()]
pub struct FinishBond<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(zero)]
    pub escrow: AccountLoader<'info, Escrow>,

    #[account(
        mut,
        constraint = creater_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = creater_token_account.owner == *admin.key,
    )]
    pub creater_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = dest_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = dest_token_account.owner == *treasury.to_account_info().key,
    )]
    pub dest_token_account: Account<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_mint_address: AccountInfo<'info>,
}

pub fn handle(ctx: Context<FinishBond>) -> Result<()> {
    let mut _escrow = ctx.accounts.escrow.load_init()?;

    msg!("apply_bond");
    let src_account_info = &mut &ctx.accounts.creater_token_account;
    let dest_account_info = &mut &ctx.accounts.dest_token_account;
    let mint_pubkey = &mut &ctx.accounts.token_mint_address.key();

    let price_update = &mut ctx.accounts.price_update;
    let maximum_age: u64 = 30;
    let feed_id: [u8; 32] =
        get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
    msg!(
        "The price is ({} ± {}) * 10^{}",
        price.price,
        price.conf,
        price.exponent
    );

    Ok(())
}
