use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use fixed::types::I80F48;
use fixed_macro::types::I80F48;
use marginfi::math_error;
use std::fmt::Formatter;
use std::fmt::Debug;
use marginfi::debug;
use marginfi::state::marginfi_group::WrappedI80F48;

pub const SECONDS_PER_YEAR: I80F48 = I80F48!(31_536_000);

pub fn cal_user_total_asset_in_marginfi(
    current_timestamp: i64,
    bank_total_asset_shares: WrappedI80F48,
    bank_total_liability_shares: WrappedI80F48,
    asset_share_value: WrappedI80F48,
    liability_share_value: WrappedI80F48,
    user_asset_shares: I80F48,
    last_update: u64,
) -> Result<I80F48> {
    let time_delta: u64 = (current_timestamp as u64 - last_update).try_into().unwrap();

    if time_delta == 0 {
        return get_asset_amount(asset_share_value.into(), user_asset_shares.into());
    }

    let total_assets = get_asset_amount(asset_share_value.into(), bank_total_asset_shares.into()).unwrap();
    let total_liabilities = get_liability_amount(liability_share_value.into(), bank_total_liability_shares.into()).unwrap();

    let changed_asset_share_value = calc_interest_rate_accrual_state_changes(time_delta, total_assets, total_liabilities, asset_share_value.into()).unwrap();

    get_asset_amount(changed_asset_share_value, user_asset_shares)
}

/// We use a simple interest rate model that auto settles the accrued interest into the lending account balances.
/// The plan is to move to a compound interest model in the future.
///
/// Simple interest rate model:
/// - `P` - principal
/// - `i` - interest rate (per second)
/// - `t` - time (in seconds)
///
/// `P_t = P_0 * (1 + i) * t`
///
/// We use two interest rates, one for lending and one for borrowing.
///
/// Lending interest rate:
/// - `i_l` - lending interest rate
/// - `i` - base interest rate
/// - `ur` - utilization rate
///
/// `i_l` = `i` * `ur`
///
pub fn calc_interest_rate_accrual_state_changes(
    time_delta: u64,
    total_assets_amount: I80F48,
    total_liabilities_amount: I80F48,
    asset_share_value: I80F48,
) -> Option<I80F48> {
    let utilization_rate = total_liabilities_amount.checked_div(total_assets_amount)?;
    let lending_apr = calc_interest_rate(utilization_rate)?;

    debug!(
        "Accruing interest for {} seconds. Utilization rate: {}. Lending APR: {}. Borrowing APR: {}. Group fee APR: {}. Insurance fee APR: {}.",
        time_delta,
        utilization_rate,
        lending_apr,
    );

    Some(calc_accrued_interest_payment_per_period(
        lending_apr,
        time_delta,
        asset_share_value,
    )?)
}

/// Piecewise linear interest rate function.
/// The curves approaches the `plateau_interest_rate` as the utilization ratio approaches the `optimal_utilization_rate`,
/// once the utilization ratio exceeds the `optimal_utilization_rate`, the curve approaches the `max_interest_rate`.
///
/// To be clear we don't particularly appreciate the piecewise linear nature of this "curve", but it is what it is.
#[inline]
fn interest_rate_curve(ur: I80F48) -> Option<I80F48> {
    let optimal_ur = I80F48!(0.6).into(); // optimal_utilization_rate.into();
    let plateau_ir = I80F48!(0.40).into(); // plateau_interest_rate.into();
    let max_ir: I80F48 = I80F48!(3).into(); // max_interest_rate.into();

    if ur <= optimal_ur {
        ur.checked_div(optimal_ur)?.checked_mul(plateau_ir)
    } else {
        (ur - optimal_ur)
            .checked_div(I80F48::ONE - optimal_ur)?
            .checked_mul(max_ir - plateau_ir)?
            .checked_add(plateau_ir)
    }
}

/// Return interest rate charged to borrowers and to depositors.
/// Rate is denominated in APR (0-).
///
/// Return (`lending_rate`, `borrowing_rate`, `group_fees_apr`, `insurance_fees_apr`)
pub fn calc_interest_rate(utilization_ratio: I80F48) -> Option<I80F48> {
    let base_rate = interest_rate_curve(utilization_ratio)?;

    // Lending rate is adjusted for utilization ratio to symmetrize payments between borrowers and depositors.
    let lending_rate = base_rate.checked_mul(utilization_ratio)?;

    assert!(lending_rate >= I80F48::ZERO);

    // TODO: Add liquidation discount check

    Some(lending_rate)
}

/// Calculates the accrued interest payment per period `time_delta` in a principal value `value` for interest rate (in APR) `arp`.
/// Result is the new principal value.
fn calc_accrued_interest_payment_per_period(
    apr: I80F48,
    time_delta: u64,
    value: I80F48,
) -> Option<I80F48> {
    let ir_per_period = apr
        .checked_mul(time_delta.into())?
        .checked_div(SECONDS_PER_YEAR)?;

    let new_value = value.checked_mul(I80F48::ONE.checked_add(ir_per_period)?)?;

    Some(new_value)
}

fn get_asset_amount(asset_share_value: I80F48, shares: I80F48) -> Result<I80F48> {
    Ok(shares
        .checked_mul(asset_share_value.into())
        .ok_or_else(math_error!())?)
}

fn get_liability_amount(liability_share_value: I80F48, value: I80F48) -> Result<I80F48> {
    Ok(value
        .checked_div(liability_share_value.into())
        .ok_or_else(math_error!())?)
}

// #[zero_copy]
// #[repr(C, align(8))]
// #[cfg_attr(
//     any(feature = "test", feature = "client"),
//     derive(PartialEq, Eq, TypeLayout)
// )]
// #[derive(Default, AnchorDeserialize, AnchorSerialize)]
// pub struct WrappedI80F48 {
//     pub value: [u8; 16],
// }

// impl Debug for WrappedI80F48 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", I80F48::from_le_bytes(self.value))
//     }
// }

// impl From<I80F48> for WrappedI80F48 {
//     fn from(i: I80F48) -> Self {
//         Self {
//             value: i.to_le_bytes(),
//         }
//     }
// }

// impl From<WrappedI80F48> for I80F48 {
//     fn from(w: WrappedI80F48) -> Self {
//         Self::from_le_bytes(w.value)
//     }
// }

