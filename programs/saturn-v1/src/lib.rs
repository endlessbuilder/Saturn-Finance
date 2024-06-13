#![allow(unused_imports)]
#![allow(unused_variables)]
use anchor_lang::prelude::*;

mod account;
mod constants;
mod error;
mod instructions;
mod jupiter_utils;
mod meteora_utils;

use account::*;
use instructions::*;
use meteora::cpi::accounts::DepositWithdrawLiquidity;
use meteora_utils::*;

declare_id!("6y1CpFjLdNfs5KUh1PfkjS11FiwH5ZrxmeMtJm9yLdJF");

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

    pub fn swap(ctx: Context<Swap>, data: Vec<u8> /*, from_amount: u64*/) -> Result<()> {
        instructions::swap::handle(ctx, data /*, from_amount*/)
    }

    pub fn meteora_deposit(
        ctx: Context<MeteoraDeposit>,
        token_amount: u64,
        minimum_lp_token_amount: u64,
    ) -> Result<()> {
        instructions::meteora_deposit::handle(ctx, token_amount, minimum_lp_token_amount)
    }

    pub fn meteora_withdraw(
        ctx: Context<MeteoraWithdraw>,
        unmint_amount: u64,
        min_out_amount: u64,
    ) -> Result<()> {
        instructions::meteora_withdraw::handle(ctx, unmint_amount, min_out_amount)
    }

    pub fn init_lending_accounts(ctx: Context<InitLendingAccount>) -> Result<()> {
        instructions::init_lending_account::handle(ctx)
    }

    pub fn handle_klend_lend(ctx: Context<KaminoLend>, amount: u64) -> Result<()> {
        instructions::handle_klend_lend::handle(ctx, amount)
    }

    pub fn handle_klend_withdraw(ctx: Context<KlendWithdraw>, amount: u64) -> Result<()> {
        instructions::handle_klend_withdraw::handle(ctx, amount)
    }
}
