use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::{error::*, kamino_utils};
use crate::account::Treasury;
use crate::constants::{TREASURY_AUTHORITY_SEED, TREASURY_METEORA_LP};
use meteora::state::Vault;


#[derive(Accounts)]
pub struct GetValueInKamino {

    pub lending_market: AccountLoader<'info, LendingMarket>,
    #[account(mut, has_one = lending_market)]
    pub withdraw_reserve: AccountLoader<'info, Reserve>,
    #[account(mut,
        has_one = lending_market,
        has_one = owner
    )]
    pub obligation: AccountLoader<'info, Obligation>,

}

pub fn handle(ctx: Context<GetValueInKamino>) -> Result<()> {
    // # get kamino value
    let clock = Clock::get();

    let obligation_amount = kamino_utils::calcu_obligation_collateral(
        lending_market,
        reserve,
        obligation,
        collateral_amount,
        clock.slot,
        ctx.accounts.withdraw_reserve.key(),
    )?;
    let liquidity_amount = kamino_utils::redeem_reserve_collateral(
        reserve,
        obligation_amount,
        clock,
        true,
    )?;

   Ok(())
}