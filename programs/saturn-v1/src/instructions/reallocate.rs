use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use std::str::FromStr;

use crate::utils::*;
use crate::{
    account::*,
    constants::{
        SEQUENCE_FLAG_SEED, SOL_PRICE_ID, TREASURY_AUTHORITY_SEED, TREASURY_SEED, USDC_MINT,
        WBTC_MINT, WBTC_PRICE_ID,
    },
};

/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct ReAllocate<'info> {
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
        constraint = sequence_flag.flag_calcu_balance == true,
        constraint = sequence_flag.flag_reallocate == false,
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

    pub price_update: Account<'info, PriceUpdateV2>,
}

#[allow(unused_variables)]
pub fn handle(ctx: Context<ReAllocate>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let kamino_balance = treasury.kamino_lend_value;
    let marignfi_balance = treasury.marginfi_lend_value;
    let meteora_balance = treasury.meteora_deposit_value;
    let usdc_balance: u64 = ctx.accounts.usdc_token_account.amount;
    let wbtc_balance: u64 = ctx.accounts.wbtc_token_account.amount;
    let sol_balance: u64 = ctx.accounts.treasury_authority.get_lamports();

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

    let total_value = treasury.treasury_value;

    let kamino_allocation: f64 = kamino_balance as f64 / total_value as f64;
    let marginfi_allocation: f64 = marignfi_balance as f64 / total_value as f64;
    let meteora_allocation: f64 = meteora_balance as f64 / total_value as f64;
    let usdc_allocation: f64 = usdc_balance as f64 / total_value as f64;
    let wbtc_allocation: f64 = wbtc_balance as f64 / total_value as f64;
    let sol_allocation: f64 = sol_balance as f64 / total_value as f64;

    let marginfi = Platform {
        id: 1,
        return_rate: 52.0,
        risk_rating: 5.0,
        allocation: marginfi_allocation,
        platform_type: 1,
    };
    let kamino = Platform {
        id: 2,
        return_rate: 32.0,
        risk_rating: 7.0,
        allocation: kamino_allocation,
        platform_type: 1,
    };
    let meteora = Platform {
        id: 3,
        return_rate: 72.0,
        risk_rating: 3.0,
        allocation: meteora_allocation,
        platform_type: 2,
    };
    let jupiterperps = Platform {
        id: 4,
        return_rate: 152.0,
        risk_rating: 8.0,
        allocation: 0.0,
        platform_type: 3,
    };
    let usdcoin = Platform {
        id: 5,
        return_rate: 1.0,
        risk_rating: 1.0,
        allocation: usdc_allocation,
        platform_type: 4,
    };
    let btc = Platform {
        id: 6,
        return_rate: 1.0,
        risk_rating: 2.0,
        allocation: wbtc_allocation,
        platform_type: 4,
    };
    let sol = Platform {
        id: 7,
        return_rate: 1.0,
        risk_rating: 4.0,
        allocation: sol_allocation,
        platform_type: 4,
    };

    let treasur = vec![marginfi, kamino, meteora, jupiterperps, usdcoin, btc, sol];
    let new_allocation = re_allocate(&treasur, PLATFORM_ALLOCATION);

    treasury.marginfi_allocation = new_allocation[0].allocation;
    treasury.kamino_allocation = new_allocation[1].allocation;
    treasury.meteora_allocation = new_allocation[2].allocation;
    treasury.jupiter_allocation = new_allocation[3].allocation;
    treasury.usdc_allocation = new_allocation[4].allocation;
    treasury.wbtc_allocation = new_allocation[5].allocation;
    treasury.sol_allocation = new_allocation[6].allocation;

    ctx.accounts.sequence_flag.flag_reallocate = true;

    Ok(())
}
