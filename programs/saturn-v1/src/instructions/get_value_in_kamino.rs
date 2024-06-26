use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use kamino_lending::obligation;
use serde::de::value;

use crate::{error::*, kamino_utils};
use crate::account::Treasury;
use crate::constants::*;
use kamino_lending::state::{LendingMarket, Reserve, Obligation};


#[derive(Accounts)]
pub struct GetValueInKamino {

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
        constraint = bonk_reserve.load()?.liquidity.mint_pubkey.to_string().as_str() == BONK_MINT,
    )]
    pub bonk_reserve: AccountLoader<'info, Reserve>,

    #[account(
        mut,
        has_one = lending_market,
        has_one = owner
    )]
    pub obligation: AccountLoader<'info, Obligation>,

}

pub fn handle(ctx: Context<GetValueInKamino>) -> Result<([u64; 3])> {
    // # get kamino value
    let clock = Clock::get();
    let obligation = &mut ctx.accounts.obligation.load_mut().unwrap();

    let sol_reserve = &mut ctx.accounts.sol_reserve.load_mut().unwrap();
    let collateral_amount1 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve.eq(sol_reserve.key()))
    .unwrap()
    .deposited_amount;
        let liquidity_amount1 = kamino_utils::redeem_reserve_collateral(
        sol_reserve,
        collateral_amount1,
        clock,
    )?;

    let usdc_reserve = &mut ctx.accounts.usdc_reserve.load_mut().unwrap();
    let collateral_amount2 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve.eq(usdc_reserve.key()))
    .unwrap()
    .deposited_amount;
    let liquidity_amount2 = kamino_utils::redeem_reserve_collateral(
        usdc_reserve,
        collateral_amount2,
        clock,
    )?;

    let bonk_reserve = &mut ctx.accounts.bonk_reserve.load_mut().unwrap();
    let collateral_amount3 = obligation
    .deposits
    .iter()
    .find(|obligation_coll| obligation_coll.deposit_reserve.eq(bonk_reserve.key()))
    .unwrap()
    .deposited_amount;
    let liquidity_amount3 = kamino_utils::redeem_reserve_collateral(
        bonk_reserve,
        collateral_amount3,
        clock,
    )?;

    let values: [u64; 3] = [0, 0, 0];
    values[0] = liquidity_amount1.into();
    values[1] = liquidity_amount2.into();
    values[2] = liquidity_amount3.into();

   Ok((values))
}