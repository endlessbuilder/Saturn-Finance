use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use fixed::types::I80F48;

use crate::account::Treasury;
use crate::{constants::*, error::*, marginfi_utils::*};
use marginfi::state::marginfi_account::MarginfiAccount;
use marginfi::state::marginfi_group::{Bank, WrappedI80F48};

#[derive(Accounts)]
pub struct GetValueInMarginFi {
    // MarginFi
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,

    #[account(
        mut,
        constraint = marginfi_account.load()?.group == marginfi_group.key(),
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    #[account(
        mut,
        constraint = bank.load()?.group == marginfi_group.key(),
        constraint = bank.load()?.mint.to_string().as_str() == SOL_MINT,
    )]
    pub sol_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = bank.load()?.group == marginfi_group.key(),
        constraint = bank.load()?.mint.to_string().as_str() == USDC_MINT,
    )]
    pub usdc_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = bank.load()?.group == marginfi_group.key(),
        constraint = bank.load()?.mint.to_string().as_str() == BONK_MINT,
    )]
    pub bonk_bank: AccountLoader<'info, Bank>,
}

pub fn handle(ctx: Context<GetValueInMarginFi>) -> Result<([u64; 3])> {
    // # get marginfi value
    let marginfi_account = &mut ctx.accounts.marginfi_account.load_mut().unwrap();
    let sol_bank = &mut ctx.accounts.sol_bank.load_mut().unwrap();
    let usdc_bank = &mut ctx.accounts.usdc_bank.load_mut().unwrap();
    let bonk_bank = &mut ctx.accounts.bonk_bank.load_mut().unwrap();
    let current_timestap = Clock::get()?.unix_timestamp;

    let lending_account = &mut marginfi_account.lending_account;

    // get sol balance
    let sol_asset_shares = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(sol_bank.key()))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap()
        .asset_shares;

    let sol_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        sol_bank.total_asset_shares,
        sol_bank.total_liability_shares,
        sol_bank.asset_share_value,
        sol_bank.liability_share_value,
        I80F48::from(sol_asset_shares),
    )
    .unwrap();

    // get usdc balance
    let usdc_asset_shares = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(usdc_bank.key()))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap()
        .asset_shares;

    let usdc_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        usdc_bank.total_asset_shares,
        usdc_bank.total_liability_shares,
        usdc_bank.asset_share_value,
        usdc_bank.liability_share_value,
        I80F48::from(usdc_asset_shares),
    )
    .unwrap();

    // get bonk balance
    let bonk_asset_shares = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(bonk_bank.key()))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap()
        .asset_shares;

    let bonk_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        bonk_bank.total_asset_shares,
        bonk_bank.total_liability_shares,
        bonk_bank.asset_share_value,
        bonk_bank.liability_share_value,
        I80F48::from(bonk_asset_shares),
    )
    .unwrap();

    let values: [u64; 3] = [0, 0, 0];
    values[0] = sol_value_in_marginfi.into(); // sol
    values[1] = usdc_value_in_marginfi.into(); // usdc
    values[2] = bonk_value_in_marginfi.into(); // bonk

    Ok((values))
}
