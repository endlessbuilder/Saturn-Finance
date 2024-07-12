use crate::{account::Treasury, constants::*, error::*, treasury};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, mint_to, MintTo, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

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

    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn handle(ctx: Context<CalcuBalance>) -> Result<()> {
    let mut treasury = &mut ctx.accounts.treasury;

    let price_update = &mut ctx.accounts.price_update;
    // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 30;
    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
    let sol_feed_id: [u8; 32] = get_feed_id_from_hex(SOL_PRICE_ID)?;
    let sol_price =
        price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &sol_feed_id)?;
    let wbtc_feed_id: [u8; 32] = get_feed_id_from_hex(WBTC_PRICE_ID)?;
    let wbtc_price =
        price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &wbtc_feed_id)?;

    let marginfi_val = treasury.marginfi_lend_value;
    let kamino_val = treasury.kamino_lend_value;
    let meteora_val = treasury.meteora_deposit_value;
    let jupiter_val = treasury.jupiter_perps_value;
    let usdc_val = ctx.accounts.usdc_token_account.amount;
    let sol_val = (ctx.accounts.treasury_authority.get_lamports() / 1_000_000_000)
        * u64::try_from(sol_price.price).unwrap() / 10u64.pow(u32::try_from(-sol_price.exponent).unwrap());
    let wbtc_val =
        ctx.accounts.wbtc_token_account.amount * u64::try_from(wbtc_price.price).unwrap() / 10u64.pow(u32::try_from(-wbtc_price.exponent).unwrap());
    treasury.treasury_value =
        marginfi_val + kamino_val + meteora_val + jupiter_val + usdc_val + sol_val + wbtc_val;

    Ok(())
}
