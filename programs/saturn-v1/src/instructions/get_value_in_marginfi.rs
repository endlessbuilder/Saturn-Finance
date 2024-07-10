use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use fixed::types::I80F48;

use crate::account::Treasury;
use crate::{constants::*, error::*, marginfi_utils::*};
use marginfi::state::marginfi_account::MarginfiAccount;
use marginfi::state::marginfi_group::{MarginfiGroup, Bank, WrappedI80F48};

#[derive(Accounts)]
pub struct GetValueInMarginFi<'info> {
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    
    // MarginFi
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,

    #[account(
        mut,
        constraint = marginfi_account.load()?.group == marginfi_group.key(),
        constraint = marginfi_account.load()?.authority == treasury_authority.key(),
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    #[account(
        mut,
        constraint = sol_bank.load()?.group == marginfi_group.key(),
        constraint = sol_bank.load()?.mint.to_string().as_str() == SOL_MINT,
    )]
    pub sol_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = usdc_bank.load()?.group == marginfi_group.key(),
        constraint = usdc_bank.load()?.mint.to_string().as_str() == USDC_MINT,
    )]
    pub usdc_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = usdt_bank.load()?.group == marginfi_group.key(),
        constraint = usdt_bank.load()?.mint.to_string().as_str() == USDT_MINT,
    )]
    pub usdt_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = wbtc_bank.load()?.group == marginfi_group.key(),
        constraint = wbtc_bank.load()?.mint.to_string().as_str() == WBTC_MINT,
    )]
    pub wbtc_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = weth_bank.load()?.group == marginfi_group.key(),
        constraint = weth_bank.load()?.mint.to_string().as_str() == WETH_MINT,
    )]
    pub weth_bank: AccountLoader<'info, Bank>,

    #[account(
        mut,
        constraint = bonk_bank.load()?.group == marginfi_group.key(),
        constraint = bonk_bank.load()?.mint.to_string().as_str() == BONK_MINT,
    )]
    pub bonk_bank: AccountLoader<'info, Bank>,
}

pub fn handle(ctx: Context<GetValueInMarginFi>) -> Result<[u64; 6]> {

    // # get marginfi value
    let marginfi_account = &mut ctx.accounts.marginfi_account.load_mut().unwrap();
    let sol_bank = &mut ctx.accounts.sol_bank.load_mut().unwrap();
    let sol_bank_pubkey = *ctx.accounts.sol_bank.to_account_info().key;
    let usdc_bank = &mut ctx.accounts.usdc_bank.load_mut().unwrap();
    let usdc_bank_pubkey = *ctx.accounts.usdc_bank.to_account_info().key;
    let usdt_bank = &mut ctx.accounts.usdt_bank.load_mut().unwrap();
    let usdt_bank_pubkey = *ctx.accounts.usdt_bank.to_account_info().key;
    let wbtc_bank = &mut ctx.accounts.wbtc_bank.load_mut().unwrap();
    let wbtc_bank_pubkey = *ctx.accounts.wbtc_bank.to_account_info().key;
    let weth_bank = &mut ctx.accounts.weth_bank.load_mut().unwrap();
    let weth_bank_pubkey = *ctx.accounts.weth_bank.to_account_info().key;
    let bonk_bank = &mut ctx.accounts.bonk_bank.load_mut().unwrap();
    let bonk_bank_pubkey = *ctx.accounts.bonk_bank.to_account_info().key;
    let current_timestap = Clock::get()?.unix_timestamp;

    let lending_account = &mut marginfi_account.lending_account;

    // get sol balance
    let sol_asset_balance = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(&sol_bank_pubkey))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap();

    let sol_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        sol_bank.total_asset_shares,
        sol_bank.total_liability_shares,
        sol_bank.asset_share_value,
        sol_bank.liability_share_value,
        I80F48::from(sol_asset_balance.asset_shares),
        sol_asset_balance.last_update,
    )
    .unwrap();

    // get usdc balance
    let usdc_asset_balance = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(&usdc_bank_pubkey))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap();

    let usdc_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        usdc_bank.total_asset_shares,
        usdc_bank.total_liability_shares,
        usdc_bank.asset_share_value,
        usdc_bank.liability_share_value,
        I80F48::from(usdc_asset_balance.asset_shares),
        usdc_asset_balance.last_update,
    )
    .unwrap();

    // get usdt balance
    let usdt_asset_balance = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(&usdt_bank_pubkey))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap();

    let usdt_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        usdt_bank.total_asset_shares,
        usdt_bank.total_liability_shares,
        usdt_bank.asset_share_value,
        usdt_bank.liability_share_value,
        I80F48::from(usdt_asset_balance.asset_shares),
        usdt_asset_balance.last_update,
    )
    .unwrap();

    // get wbtc balance
    let wbtc_asset_balance = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(&wbtc_bank_pubkey))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap();

    let wbtc_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        wbtc_bank.total_asset_shares,
        wbtc_bank.total_liability_shares,
        wbtc_bank.asset_share_value,
        wbtc_bank.liability_share_value,
        I80F48::from(wbtc_asset_balance.asset_shares),
        wbtc_asset_balance.last_update,
    )
    .unwrap();

    // get weth balance
    let weth_asset_balance = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(&weth_bank_pubkey))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap();

    let weth_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        weth_bank.total_asset_shares,
        weth_bank.total_liability_shares,
        weth_bank.asset_share_value,
        weth_bank.liability_share_value,
        I80F48::from(weth_asset_balance.asset_shares),
        weth_asset_balance.last_update,
    )
    .unwrap();

    // get bonk balance
    let bonk_asset_balance = lending_account
        .balances
        .iter_mut()
        .find(|balance| balance.active && balance.bank_pk.eq(&bonk_bank_pubkey))
        .ok_or_else(|| error!(MarginfiError::BankAccoutNotFound))
        .unwrap();

    let bonk_value_in_marginfi = cal_user_total_asset_in_marginfi(
        current_timestap,
        bonk_bank.total_asset_shares,
        bonk_bank.total_liability_shares,
        bonk_bank.asset_share_value,
        bonk_bank.liability_share_value,
        I80F48::from(bonk_asset_balance.asset_shares),
        bonk_asset_balance.last_update,
    )
    .unwrap();

    let mut values: [u64; 6] = [0, 0, 0, 0, 0, 0];
    values[0] = sol_value_in_marginfi.to_num::<u64>(); // sol
    // values[1] = usdc_value_in_marginfi.into(); // usdc
    // values[2] = usdt_value_in_marginfi.into(); // usdt
    // values[3] = wbtc_value_in_marginfi.into(); // wbtc
    // values[4] = weth_value_in_marginfi.into(); // weth
    // values[5] = bonk_value_in_marginfi.into(); // bonk

    Ok(values)
}
