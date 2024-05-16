use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    account::{Escrow, Treasury},
    constants::*,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
        payer = admin,
        space = 40
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.treasury = ctx.accounts.admin.key();
    Ok(())
}
