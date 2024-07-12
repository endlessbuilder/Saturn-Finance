use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use std::str::FromStr;
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

use crate::utils::*;
use crate::{
    account::*,
    constants::{TREASURY_AUTHORITY_SEED, TREASURY_SEED, USDC_MINT, WBTC_MINT},
};

/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct ReAllocate<'info> {
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

}

#[allow(unused_variables)]
pub fn handle(ctx: Context<ReAllocate>) -> Result<()> {
    let mut treasury = &mut ctx.accounts.treasury;
    let kamino_balance = treasury.kamino_lend_amount;
    let marignfi_balance = treasury.marginfi_lend_amount;
    let meteora_balance = treasury.meteora_deposit_amount;
    let usdc_balance: u64 = ctx.accounts.usdc_token_account.amount;
    let wbtc_balance: u64 = ctx.accounts.wbtc_token_account.amount;
    let sol_balance: u64 = ctx.accounts.treasury_authority.get_lamports();


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
    

    Ok(())
}
