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
// use state::marginfi_group::{BankConfigCompact, BankConfigOpt};
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

    // User instructions

    /// Initialize a marginfi account for a given group
    #[allow(unused_variables)]
    pub fn marginfi_account_initialize(ctx: Context<MarginfiAccountInitialize>) -> MarginfiResult {
        // marginfi_account::initialize_account(ctx)
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn lending_account_deposit(
        ctx: Context<LendingAccountDeposit>,
        amount: u64,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_deposit(ctx, amount)
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn lending_account_withdraw(
        ctx: Context<LendingAccountWithdraw>,
        amount: u64,
        withdraw_all: Option<bool>,
    ) -> MarginfiResult {
        // marginfi_account::lending_account_withdraw(ctx, amount, withdraw_all)
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
