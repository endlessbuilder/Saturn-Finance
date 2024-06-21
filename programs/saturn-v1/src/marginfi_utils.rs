use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};


// Calculate the interest rate accrual state changes for a given time period
//
// Collected protocol and insurance fees are stored in state.
// A separate instruction is required to withdraw these fees.
pub fn accrue_interest(
    &mut self,
    current_timestamp: i64,
    #[cfg(not(feature = "client"))] bank: Pubkey,
) -> MarginfiResult<()> {
    #[cfg(all(not(feature = "client"), feature = "debug"))]
    solana_program::log::sol_log_compute_units();

    let time_delta: u64 = (current_timestamp - self.last_update).try_into().unwrap();

    if time_delta == 0 {
        return Ok(());
    }

    let total_assets = self.get_asset_amount(self.total_asset_shares.into())?;
    let total_liabilities = self.get_liability_amount(self.total_liability_shares.into())?;

    self.last_update = current_timestamp;

    if (total_assets == I80F48::ZERO) || (total_liabilities == I80F48::ZERO) {
        #[cfg(not(feature = "client"))]
        emit!(LendingPoolBankAccrueInterestEvent {
            header: GroupEventHeader {
                marginfi_group: self.group,
                signer: None
            },
            bank,
            mint: self.mint,
            delta: time_delta,
            fees_collected: 0.,
            insurance_collected: 0.,
        });

        return Ok(());
    }

    let (asset_share_value, liability_share_value, fees_collected, insurance_collected) =
        calc_interest_rate_accrual_state_changes(
            time_delta,
            total_assets,
            total_liabilities,
            &self.config.interest_rate_config,
            self.asset_share_value.into(),
            self.liability_share_value.into(),
        )
        .ok_or_else(math_error!())?;

    debug!("deposit share value: {}\nliability share value: {}\nfees collected: {}\ninsurance collected: {}",
            asset_share_value, liability_share_value, fees_collected, insurance_collected);

    self.asset_share_value = asset_share_value.into();
    self.liability_share_value = liability_share_value.into();

    self.collected_group_fees_outstanding = {
        fees_collected
            .checked_add(self.collected_group_fees_outstanding.into())
            .ok_or_else(math_error!())?
            .into()
    };

    self.collected_insurance_fees_outstanding = {
        insurance_collected
            .checked_add(self.collected_insurance_fees_outstanding.into())
            .ok_or_else(math_error!())?
            .into()
    };

    #[cfg(not(feature = "client"))]
    {
        #[cfg(feature = "debug")]
        solana_program::log::sol_log_compute_units();

        emit!(LendingPoolBankAccrueInterestEvent {
            header: GroupEventHeader {
                marginfi_group: self.group,
                signer: None
            },
            bank,
            mint: self.mint,
            delta: time_delta,
            fees_collected: fees_collected.to_num::<f64>(),
            insurance_collected: insurance_collected.to_num::<f64>(),
        });
    }

    Ok(())
}
