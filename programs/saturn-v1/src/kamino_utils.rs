use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use kamino_lending::state::{LendingMarket, Reserve, Obligation};
use kamino_lending::LendingError;

pub fn withdraw_obligation_collateral(
    lending_market: &LendingMarket,
    withdraw_reserve: &mut Reserve,
    obligation: &mut Obligation,
    collateral_amount: u64,
    slot: Slot,
    withdraw_reserve_pk: Pubkey,
) -> Result<u64> {
    if collateral_amount == 0 {
        return err!(LendingError::InvalidAmount);
    }

    let is_borrows_empty = obligation.borrows_empty();

    let required_price_status = if is_borrows_empty {
        PriceStatusFlags::NONE
    } else {
        PriceStatusFlags::ALL_CHECKS
    };

    if withdraw_reserve
        .last_update
        .is_stale(slot, required_price_status)?
    {
        msg!(
            "Withdraw reserve is stale and must be refreshed in the current slot, price status: {:08b}",
            withdraw_reserve.last_update.get_price_status().0
        );
        return err!(LendingError::ReserveStale);
    }

    if obligation
        .last_update
        .is_stale(slot, required_price_status)?
    {
        msg!(
            "Obligation is stale and must be refreshed in the current slot, price status: {:08b}",
            obligation.last_update.get_price_status().0
        );
        return err!(LendingError::ObligationStale);
    }

    let collateral_index = obligation.position_of_collateral_in_deposits(withdraw_reserve_pk)?;
    let collateral = &obligation.deposits[collateral_index];
    if collateral.deposited_amount == 0 {
        return err!(LendingError::ObligationCollateralEmpty);
    }

    check_elevation_group_borrowing_enabled(lending_market, obligation)?;

    if obligation.num_of_obsolete_reserves > 0
        && withdraw_reserve.config.status() == ReserveStatus::Active
    {
        return err!(LendingError::ObligationInDeprecatedReserve);
    }

    let withdraw_amount = if is_borrows_empty {
        if collateral_amount == u64::MAX {
            collateral.deposited_amount
        } else {
            collateral.deposited_amount.min(collateral_amount)
        }
    } else if obligation.deposited_value_sf == 0 {
        msg!("Obligation deposited value is zero");
        return err!(LendingError::ObligationDepositsZero);
    } else {
        let (reserve_loan_to_value_pct, _) = get_max_ltv_and_liquidation_threshold(
            withdraw_reserve,
            get_elevation_group(obligation.elevation_group, lending_market)?,
        )?;

        let max_withdraw_value = obligation.max_withdraw_value(reserve_loan_to_value_pct)?;

        if max_withdraw_value == Fraction::ZERO {
            msg!("Maximum withdraw value is zero");
            return err!(LendingError::WithdrawTooLarge);
        }

        let collateral_value = Fraction::from_bits(collateral.market_value_sf);
        let withdraw_amount = if collateral_amount == u64::MAX {
            let withdraw_value = max_withdraw_value.min(collateral_value);
            let withdraw_ratio = withdraw_value / collateral_value;

            let ratioed_amount_f = withdraw_ratio * u128::from(collateral.deposited_amount);
            let ratioed_amount: u64 = ratioed_amount_f.to_floor();

            min(collateral.deposited_amount, ratioed_amount)
        } else {
            let withdraw_amount = collateral_amount.min(collateral.deposited_amount);
            let withdraw_ratio =
                Fraction::from(withdraw_amount) / u128::from(collateral.deposited_amount);
            let withdraw_value = collateral_value * withdraw_ratio;
            if withdraw_value > max_withdraw_value {
                msg!("Withdraw value cannot exceed maximum withdraw value, collateral_amount={}, collateral.deposited_amount={} withdraw_pct={}, collateral_value={}, max_withdraw_value={} withdraw_value={}",
                    collateral_amount,
                    collateral.deposited_amount,
                    withdraw_ratio,
                    collateral_value,
                    max_withdraw_value,
                    withdraw_value);
                return err!(LendingError::WithdrawTooLarge);
            }
            withdraw_amount
        };

        if withdraw_amount == 0 {
            msg!("Withdraw amount is too small to transfer collateral");
            return err!(LendingError::WithdrawTooSmall);
        }
        withdraw_amount
    };

    let previous_debt_in_elevation_group =
        collateral.borrowed_amount_against_this_collateral_in_elevation_group;
    let is_full_withdrawal = obligation.withdraw(withdraw_amount, collateral_index)?;
    obligation.last_update.mark_stale();

    if is_full_withdrawal == WithdrawResult::Full {
        utils::update_elevation_group_debt_trackers_on_full_withdraw(
            previous_debt_in_elevation_group,
            obligation.elevation_group,
            withdraw_reserve,
        )?;
    }

    post_withdraw_obligation_invariants(
        withdraw_reserve
            .collateral_exchange_rate()?
            .fraction_collateral_to_liquidity(Fraction::from(withdraw_amount)),
        obligation,
        withdraw_reserve,
        Fraction::from_bits(obligation.deposits[collateral_index].market_value_sf),
        Fraction::from_bits(lending_market.min_net_value_in_obligation_sf),
    )?;

    Ok(withdraw_amount)
}

pub fn redeem_reserve_collateral(
    reserve: &mut Reserve,
    collateral_amount: u64,
    clock: &Clock,
    add_amount_to_withdrawal_caps: bool,
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

    let liquidity_amount = reserve.redeem_collateral(collateral_amount)?;
    refresh_reserve_limit_timestamps(reserve, clock.slot)?;
    reserve.last_update.mark_stale();

    if add_amount_to_withdrawal_caps {
        add_to_withdrawal_accum(
            &mut reserve.config.deposit_withdrawal_cap,
            liquidity_amount,
            u64::try_from(clock.unix_timestamp).unwrap(),
        )?;
    }

    Ok(liquidity_amount)
}