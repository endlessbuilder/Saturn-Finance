use anchor_lang::prelude::*;
use anchor_spl::token::{self, mint_to, MintTo, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

use crate::constants::{
    SEQUENCE_FLAG_SEED, SOL_PRICE_ID, TREASURY_AUTHORITY_SEED, TREASURY_SEED, USDC_MINT,
    USDC_PRICE_ID, WBTC_MINT, WBTC_PRICE_ID,
};
use crate::{
    account::{SequenceFlag, Treasury},
    error::*,
};

#[derive(Accounts)]
pub struct CalcuBalance<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury_authority.key()
    )]
    signer: Signer<'info>,
    /// user CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [SEQUENCE_FLAG_SEED.as_ref()],
        bump,
    )]
    pub sequence_flag: Account<'info, SequenceFlag>,

    #[account(
        mut,
        token::mint = Pubkey::from_str(USDC_MINT).unwrap(),
    )]
    pub usdc_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = Pubkey::from_str(WBTC_MINT).unwrap(),
    )]
    pub wbtc_token_account: Account<'info, TokenAccount>,

    pub sol_price_update: Account<'info, PriceUpdateV2>,
    pub usdc_price_update: Account<'info, PriceUpdateV2>,
    pub wbtc_price_update: Account<'info, PriceUpdateV2>,
}

pub fn handle(ctx: Context<CalcuBalance>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;

    let sol_price_update = &mut ctx.accounts.sol_price_update;
    // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 30;
    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
    let sol_feed_id: [u8; 32] = get_feed_id_from_hex(SOL_PRICE_ID)?;
    let sol_price =
        sol_price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &sol_feed_id)?;

    let usdc_price_update = &mut ctx.accounts.usdc_price_update;
    let usdc_feed_id: [u8; 32] = get_feed_id_from_hex(USDC_PRICE_ID)?;
    let usdc_price =
        usdc_price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &usdc_feed_id)?;

    let wbtc_price_update = &mut ctx.accounts.wbtc_price_update;
    let wbtc_feed_id: [u8; 32] = get_feed_id_from_hex(WBTC_PRICE_ID)?;
    let wbtc_price =
        wbtc_price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &wbtc_feed_id)?;

    let marginfi_val = treasury.marginfi_lend_value;
    let kamino_val = treasury.kamino_lend_value;
    let meteora_val = treasury.meteora_deposit_value;
    let jupiter_val = treasury.jupiter_perps_value;

    let sol_val = (ctx.accounts.treasury_authority.get_lamports() / 1_000_000_000)
        * u64::try_from(sol_price.price).unwrap()
        * 10u64.pow(u32::try_from(sol_price.exponent).unwrap());

    let usdc_val = (ctx.accounts.usdc_token_account.amount / 1_000_000)
        * u64::try_from(usdc_price.price).unwrap()
        * 10u64.pow(u32::try_from(sol_price.exponent).unwrap());

    let wbtc_val = (ctx.accounts.wbtc_token_account.amount / 100_000_000)
        * u64::try_from(wbtc_price.price).unwrap()
        * 10u64.pow(u32::try_from(wbtc_price.exponent).unwrap());

    treasury.sol_value = sol_val as f64;
    treasury.usdc_value = usdc_val as f64;
    treasury.wbtc_value = wbtc_val as f64;
    treasury.treasury_value =
        marginfi_val + kamino_val + meteora_val + jupiter_val + sol_val as f64 + usdc_val as f64 + wbtc_val as f64;

    let sequence_flag = &mut ctx.accounts.sequence_flag;
    sequence_flag.flag_calcu_balance = true;
    sequence_flag.flag_reallocate = false;
    sequence_flag.flag_marginfi = false;
    sequence_flag.flag_kamino = false;
    sequence_flag.flag_meteora = false;
    sequence_flag.flag_jupiter = false;

    Ok(())
}
