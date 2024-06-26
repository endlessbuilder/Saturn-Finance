use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use kamino_lending::state::{LendingMarket, Reserve, Obligation};
use kamino_lending::LendingError;
use kamino_lending::state::*;

pub fn redeem_reserve_collateral(
    reserve: &mut Reserve,
    collateral_amount: u64,
    clock: &Clock,
) -> Result<u64> {
    if collateral_amount == 0 {
        msg!("Collateral amount provided cannot be zero");
        return err!(LendingError::InvalidAmount);
    }

    if reserve
        .last_update
        .is_stale(clock.slot, PriceStatusFlags::NONE)?
    {
        msg!("Reserve is stale and must be refreshed in the current slot");
        return err!(LendingError::ReserveStale);
    }

    let liquidity_amount = redeem_collateral(reserve, collateral_amount)?;
    refresh_reserve_limit_timestamps(reserve, clock.slot)?;
    reserve.last_update.mark_stale();

    Ok(liquidity_amount)
}

pub fn redeem_collateral(reserve: &mut Reserve, collateral_amount: u64) -> Result<u64> {
    let collateral_exchange_rate = reserve.collateral_exchange_rate()?;

    let liquidity_amount = collateral_exchange_rate.collateral_to_liquidity(collateral_amount);

    Ok(liquidity_amount)
}

