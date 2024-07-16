use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    account::{Escrow, Treasury}, constants::*, sequence_flag, SequenceFlag
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
        space = 184
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        init,
        seeds = [SEQUENCE_FLAG_SEED.as_ref()],
        bump,
        payer = admin,
        space = 7
    )]
    pub sequence_flag: Account<'info, SequenceFlag>,

    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    treasury.treasury_admin = ctx.accounts.admin.key();
    treasury.staking_index = 1; // STF index starts at 1 sSTF = STF
    treasury.token_minted = 100 * 100; // we assume STF decimal = 2
    treasury.treasury_value = 1000 * 1_000_000; // 1000 USDT
    treasury.token_staked = 0;

    let sequence_flag = &mut ctx.accounts.sequence_flag;
    sequence_flag.flag_calcu_balance = false;
    sequence_flag.flag_reallocate = false;
    sequence_flag.flag_marginfi = false;
    sequence_flag.flag_kamino = false;
    sequence_flag.flag_meteora = false;
    sequence_flag.flag_jupiter = false;
    sequence_flag.flag_swap = false;

    Ok(())
}
