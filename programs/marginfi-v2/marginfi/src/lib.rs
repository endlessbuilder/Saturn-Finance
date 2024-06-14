pub mod constants;
pub mod errors;
pub mod events;
pub mod macros;
pub mod prelude;
pub mod state;
pub mod utils;
pub mod context;

use anchor_lang::prelude::*;
use prelude::*;
use state::marginfi_group::{BankConfigCompact, BankConfigOpt};
use context::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "mainnet-beta")] {
        declare_id!("MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA");
    } else if #[cfg(feature = "devnet")] {
        declare_id!("neetcne3Ctrrud7vLdt2ypMm21gZHGN2mCmqWaMVcBQ");
    } else {
        declare_id!("neetcne3Ctrrud7vLdt2ypMm21gZHGN2mCmqWaMVcBQ");
    }
}

#[program]
pub mod marginfi {
    use super::*;

    pub fn marginfi_group_initialize(ctx: Context<MarginfiGroupInitialize>) -> MarginfiResult {
        Ok(())
    }

    pub fn marginfi_group_configure(
        ctx: Context<MarginfiGroupConfigure>,
        config: GroupConfig,
    ) -> MarginfiResult {
        Ok(())
    }

    pub fn lending_pool_add_bank(
        ctx: Context<LendingPoolAddBank>,
        bank_config: BankConfigCompact,
    ) -> MarginfiResult {
        Ok(())
    }

    /// A copy of lending_pool_add_bank with an additional bank seed.
    /// This seed is used to create a PDA for the bank's signature.
    /// lending_pool_add_bank is preserved for backwards compatibility.
    pub fn lending_pool_add_bank_with_seed(
        ctx: Context<LendingPoolAddBankWithSeed>,
        bank_config: BankConfigCompact,
        bank_seed: u64,
    ) -> MarginfiResult {
        Ok(())
    }

    pub fn lending_pool_configure_bank(
        ctx: Context<LendingPoolConfigureBank>,
        bank_config_opt: BankConfigOpt,
    ) -> MarginfiResult {
        Ok(())
    }

    pub fn lending_pool_setup_emissions(
        ctx: Context<LendingPoolSetupEmissions>,
        flags: u64,
        rate: u64,
        total_emissions: u64,
    ) -> MarginfiResult {
        Ok(())
    }

    pub fn lending_pool_update_emissions_parameters(
        ctx: Context<LendingPoolUpdateEmissionsParameters>,
        emissions_flags: Option<u64>,
        emissions_rate: Option<u64>,
        additional_emissions: Option<u64>,
    ) -> MarginfiResult {
        // marginfi_group::lending_pool_update_emissions_parameters(
        //     ctx,
        //     emissions_flags,
        //     emissions_rate,
        //     additional_emissions,
        // )
        Ok(())
    }

    /// Handle bad debt of a bankrupt marginfi account for a given bank.
    pub fn lending_pool_handle_bankruptcy(
        ctx: Context<LendingPoolHandleBankruptcy>,
    ) -> MarginfiResult {
        // marginfi_group::lending_pool_handle_bankruptcy(ctx)
        Ok(())
    }

    // User instructions

    /// Initialize a marginfi account for a given group
    pub fn marginfi_account_initialize(ctx: Context<MarginfiAccountInitialize>) -> MarginfiResult {
        // marginfi_account::initialize_account(ctx)
        Ok(())
    }

    pub fn lending_account_deposit(
        ctx: Context<LendingAccountDeposit>,
        amount: u64,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_deposit(ctx, amount)
        Ok(())
    }

    pub fn lending_account_repay(
        ctx: Context<LendingAccountRepay>,
        amount: u64,
        repay_all: Option<bool>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_repay(ctx, amount, repay_all)
        Ok(())
    }

    pub fn lending_account_withdraw(
        ctx: Context<LendingAccountWithdraw>,
        amount: u64,
        withdraw_all: Option<bool>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_withdraw(ctx, amount, withdraw_all)
        Ok(())
    }

    pub fn lending_account_borrow(
        ctx: Context<LendingAccountBorrow>,
        amount: u64,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_borrow(ctx, amount)
        Ok(())
    }

    pub fn lending_account_close_balance(
        ctx: Context<LendingAccountCloseBalance>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_close_balance(ctx)
        Ok(())
    }

    pub fn lending_account_withdraw_emissions(
        ctx: Context<LendingAccountWithdrawEmissions>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_withdraw_emissions(ctx)
        Ok(())
    }

    pub fn lending_account_settle_emissions(
        ctx: Context<LendingAccountSettleEmissions>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_settle_emissions(ctx)
        Ok(())
    }

    /// Liquidate a lending account balance of an unhealthy marginfi account
    pub fn lending_account_liquidate(
        ctx: Context<LendingAccountLiquidate>,
        asset_amount: u64,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_liquidate(ctx, asset_amount)
        Ok(())
    }

    pub fn lending_account_start_flashloan(
        ctx: Context<LendingAccountStartFlashloan>,
        end_index: u64,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_start_flashloan(ctx, end_index)
        Ok(())
    }

    pub fn lending_account_end_flashloan(
        ctx: Context<LendingAccountEndFlashloan>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_end_flashloan(ctx)
        Ok(())
    }

    // Operational instructions
    pub fn lending_pool_accrue_bank_interest(
        ctx: Context<LendingPoolAccrueBankInterest>,
    ) -> MarginfiResult {
        // marginfi_group::lending_pool_accrue_bank_interest(ctx)
        Ok(())
    }

    pub fn lending_pool_collect_bank_fees(
        ctx: Context<LendingPoolCollectBankFees>,
    ) -> MarginfiResult {
        // marginfi_group::lending_pool_collect_bank_fees(ctx)
        Ok(())
    }

    pub fn lending_pool_withdraw_fees(
        ctx: Context<LendingPoolWithdrawFees>,
        amount: u64,
    ) -> MarginfiResult {
        // marginfi_group::lending_pool_withdraw_fees(ctx, amount)
        Ok(())
    }

    pub fn lending_pool_withdraw_insurance(
        ctx: Context<LendingPoolWithdrawInsurance>,
        amount: u64,
    ) -> MarginfiResult {
        // marginfi_group::lending_pool_withdraw_insurance(ctx, amount)
        Ok(())
    }

    pub fn set_account_flag(ctx: Context<SetAccountFlag>, flag: u64) -> MarginfiResult {
        // marginfi_group::set_account_flag(ctx, flag)
        Ok(())
    }

    pub fn unset_account_flag(ctx: Context<UnsetAccountFlag>, flag: u64) -> MarginfiResult {
        // marginfi_group::unset_account_flag(ctx, flag)
        Ok(())
    }

    pub fn set_new_account_authority(
        ctx: Context<MarginfiAccountSetAccountAuthority>,
    ) -> MarginfiResult {
        // marginfi_account::set_account_transfer_authority(ctx)
        Ok(())
    }
}

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;
#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "marginfi v2",
    project_url: "https://app.marginfi.com/",
    contacts: "email:security@mrgn.group",
    policy: "https://github.com/mrgnlabs/marginfi-v2/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/mrgnlabs/marginfi-v2"
}
