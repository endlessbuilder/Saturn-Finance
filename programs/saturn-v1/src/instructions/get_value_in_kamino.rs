use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use kamino_lending::obligation;
use serde::de::value;

use crate::{error::*, kamino_utils, treasury};
use crate::account::Treasury;
use crate::constants::*;
use kamino_lending::state::{LendingMarket, Reserve, Obligation};


#[derive(Accounts)]
pub struct GetValueInKamino<'info> {
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    #[account(
        mut, 
        has_one = lending_market,
        constraint = sol_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == SOL_MINT,
    )]
    pub sol_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        has_one = lending_market,
        constraint = usdc_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == USDC_MINT,
    )]
    pub usdc_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        has_one = lending_market,
        constraint = usdt_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == USDT_MINT,
    )]
    pub usdt_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        has_one = lending_market,
        constraint = wbtc_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == WBTC_MINT,
    )]
    pub wbtc_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        has_one = lending_market,
        constraint = weth_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == WETH_MINT,
    )]
    pub weth_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut, 
        has_one = lending_market,
        constraint = bonk_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == BONK_MINT,
    )]
    pub bonk_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut,
        has_one = lending_market,
        constraint = obligation.load()?.owner == treasury_authority.key(),
    )]
    pub obligation: AccountLoader<'info, Obligation>,

}

pub fn handle(ctx: Context<GetValueInKamino>) -> Result<[u64; 6]> {
    // # get kamino value
    let clock = Clock::get().unwrap();
    let obligation = &mut ctx.accounts.obligation.load_mut().unwrap();
    // sol
    let sol_reserve = &mut ctx.accounts.sol_reserve.load_mut().unwrap();
    let sol_reserve_pubkey = *ctx.accounts.sol_reserve.to_account_info().key;
    let collateral_amount1 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve == sol_reserve_pubkey)
    .unwrap()
    .deposited_amount;
        let liquidity_amount1 = kamino_utils::redeem_reserve_collateral(
        sol_reserve,
        collateral_amount1,
        &clock,
    )?;
    // usdc
    let usdc_reserve = &mut ctx.accounts.usdc_reserve.load_mut().unwrap();
    let usdc_reserve_pubkey = *ctx.accounts.sol_reserve.to_account_info().key;
    let collateral_amount2 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve == usdc_reserve_pubkey)
    .unwrap()
    .deposited_amount;
    let liquidity_amount2 = kamino_utils::redeem_reserve_collateral(
        usdc_reserve,
        collateral_amount2,
        &clock,
    )?;
    ctx.accounts.treasury.kamino_lend_assets = collateral_amount2;

    // usdt
    let usdt_reserve = &mut ctx.accounts.usdt_reserve.load_mut().unwrap();
    let usdt_reserve_pubkey = *ctx.accounts.usdt_reserve.to_account_info().key;
    let collateral_amount3 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve == usdt_reserve_pubkey)
    .unwrap()
    .deposited_amount;
    let liquidity_amount3 = kamino_utils::redeem_reserve_collateral(
        usdt_reserve,
        collateral_amount3,
        &clock,
    )?;
    // wbtc
    let wbtc_reserve = &mut ctx.accounts.wbtc_reserve.load_mut().unwrap();
    let wbtc_reserve_pubkey = *ctx.accounts.wbtc_reserve.to_account_info().key;
    let collateral_amount4 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve == wbtc_reserve_pubkey)
    .unwrap()
    .deposited_amount;
    let liquidity_amount4 = kamino_utils::redeem_reserve_collateral(
        wbtc_reserve,
        collateral_amount4,
        &clock,
    )?;
    // weth
    let weth_reserve = &mut ctx.accounts.weth_reserve.load_mut().unwrap();
    let weth_reserve_pubkey = *ctx.accounts.weth_reserve.to_account_info().key;
    let collateral_amount5 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve == weth_reserve_pubkey)
    .unwrap()
    .deposited_amount;
    let liquidity_amount5 = kamino_utils::redeem_reserve_collateral(
        weth_reserve,
        collateral_amount5,
        &clock,
    )?;
    //bonk
    let bonk_reserve = &mut ctx.accounts.bonk_reserve.load_mut().unwrap();
    let bonk_reserve_pubkey = *ctx.accounts.bonk_reserve.to_account_info().key;
    let collateral_amount6 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve == bonk_reserve_pubkey)
    .unwrap()
    .deposited_amount;
    let liquidity_amount6 = kamino_utils::redeem_reserve_collateral(
        bonk_reserve,
        collateral_amount6,
        &clock,
    )?;

    let mut values: [u64; 6] = [0, 0, 0, 0, 0, 0];
    values[0] = liquidity_amount1.into(); // sol
    values[1] = liquidity_amount2.into(); // usdc
    values[2] = liquidity_amount3.into(); // usdt
    values[3] = liquidity_amount4.into(); // wbtc
    values[4] = liquidity_amount5.into(); // weth
    values[5] = liquidity_amount6.into(); // bonk

    ctx.accounts.treasury.kamino_lend_value = values.iter().sum();

   Ok(values)
}