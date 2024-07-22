#![allow(unused_imports)]
#![allow(unused_variables)]
use anchor_lang::prelude::*;

mod account;
mod constants;
mod error;
mod instructions;
mod jupiter_utils;
mod meteora_utils;
mod kamino_utils;
mod marginfi_utils;
mod utils;

use account::*;
use instructions::*;

declare_id!("Fby86A5qXMFMhpjmneseHYfoC5FrMrtaihxYV7jXEuFz");

#[allow(unused_variables)]
#[program]
pub mod saturn_v_1 {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handle(ctx)
    }

    // Create Bond and Pay
    pub fn apply_bond(ctx: Context<ApplyBond>, args: ApplyBondArgs) -> Result<()> {
        instructions::apply_bond::handle(ctx, args.token_amount, args.spot_price)
    }

    // Redeem Bond after period is over.
    pub fn finish_bond(ctx: Context<FinishBond>) -> Result<()> {
        instructions::finish_bond::handle(ctx)
    }

    // Amount to stake in STF token (9 Decimals)
    pub fn stake_stf(ctx: Context<StakeSTF>, amount_to_stake: u64) -> Result<()> {
        instructions::stake::handle(ctx, amount_to_stake)
    }

    // Amount to unstake in staking index not in STF token
    pub fn unstake_stf(ctx: Context<UnStakeSTF>, amount_to_unstake: u64) -> Result<()> {
        instructions::unstake::handle(ctx, amount_to_unstake)
    }

    pub fn swap(ctx: Context<Swap>, data: Vec<u8> , from_amount: u64) -> Result<()> {
        instructions::swap::handle(ctx, data , from_amount)
    }

    pub fn meteora_deposit(
        ctx: Context<MeteoraDeposit>,
        pool_token_amount: u64,
        maximum_token_a_amount: u64,
        maximum_token_b_amount: u64,
    ) -> Result<()> {
        instructions::meteora_deposit::handle(ctx, pool_token_amount, maximum_token_a_amount, maximum_token_b_amount)
    }

    pub fn meteora_withdraw(
        ctx: Context<MeteoraWithdraw>,
        pool_token_amount: u64,
        maximum_token_a_amount: u64,
        maximum_token_b_amount: u64,
    ) -> Result<()> {
        instructions::meteora_withdraw::handle(ctx, pool_token_amount, maximum_token_a_amount, maximum_token_b_amount)
    }

    pub fn init_lending_accounts(ctx: Context<InitLendingAccount>) -> Result<()> {
        instructions::init_lending_account::handle(ctx)
    }

    pub fn klend_lend(ctx: Context<KaminoLend>) -> Result<()> {
        instructions::klend_lend::handle(ctx)
    }

    pub fn klend_withdraw(ctx: Context<KlendWithdraw>) -> Result<()> {
        instructions::klend_withdraw::handle(ctx)
    }

    pub fn marginfi_lend(ctx: Context<MarginfiLend>) -> Result<()> {
        instructions::marginfi_lend::handle(ctx)
    }

    pub fn marginfi_withdraw(ctx: Context<MarginfiWithdraw>) -> Result<()> {
        instructions::marginfi_withdraw::handle(ctx)
    }

    pub fn get_value_in_meteora(ctx: Context<GetValueInMeteora>) -> Result<f64> {
        instructions::get_value_in_meteora::handle(ctx)
    }

    pub fn get_value_in_kamino(ctx: Context<GetValueInKamino>) -> Result<[f64; 6]> {
        instructions::get_value_in_kamino::handle(ctx)
    }

    pub fn get_value_in_marginfi(ctx: Context<GetValueInMarginFi>) -> Result<[f64; 6]> {
        instructions::get_value_in_marginfi::handle(ctx)
    }

    pub fn calcu_balance(ctx: Context<CalcuBalance>) -> Result<()> {
        instructions::calcu_balance::handle(ctx)
    }

    pub fn reallocate(ctx: Context<ReAllocate>, return_rate: [f64; 7], risk_rating: [f64; 7]) -> Result<()> {
        instructions::reallocate::handle(ctx, return_rate, risk_rating)
    }

    pub fn cashingout_reedem(ctx: Context<CashingoutReedem>, amount: u64) -> Result<()> {
        instructions::cashingout_reedem::handle(ctx, amount)
    }
    
}
