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
        constraint = signer.key() == treasury.treasury_admin.key()
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

}

#[allow(unused_variables)]
pub fn handle(ctx: Context<ReAllocate>, return_rate: [f64; 7], risk_rating: [f64; 7]) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    
    let kamino_balance = treasury.kamino_lend_value;
    let marignfi_balance = treasury.marginfi_lend_value;
    let meteora_balance = treasury.meteora_deposit_value;

    let usdc_balance: f64 = treasury.usdc_value;
    let wbtc_balance: f64 = treasury.wbtc_value;
    let sol_balance: f64 = treasury.sol_value;

    let total_value = treasury.treasury_value;

    let kamino_allocation: f64 = kamino_balance as f64 / total_value as f64;
    let marginfi_allocation: f64 = marignfi_balance as f64 / total_value as f64;
    let meteora_allocation: f64 = meteora_balance as f64 / total_value as f64;
    let usdc_allocation: f64 = usdc_balance as f64 / total_value as f64;
    let wbtc_allocation: f64 = wbtc_balance as f64 / total_value as f64;
    let sol_allocation: f64 = sol_balance as f64 / total_value as f64;

    let marginfi = Platform {
        id: 1,
        return_rate: return_rate[0],
        risk_rating: risk_rating[0],
        allocation: marginfi_allocation,
        platform_type: 1,
    };
    let kamino = Platform {
        id: 2,
        return_rate: return_rate[1],
        risk_rating: risk_rating[1],
        allocation: kamino_allocation,
        platform_type: 1,
    };
    let meteora = Platform {
        id: 3,
        return_rate: return_rate[2],
        risk_rating: risk_rating[2],
        allocation: meteora_allocation,
        platform_type: 2,
    };
    let jupiterperps = Platform {
        id: 4,
        return_rate: return_rate[3],
        risk_rating: risk_rating[3],
        allocation: 0.0,
        platform_type: 3,
    };
    let usdcoin = Platform {
        id: 5,
        return_rate: return_rate[4],
        risk_rating: risk_rating[4],
        allocation: usdc_allocation,
        platform_type: 4,
    };
    let btc = Platform {
        id: 6,
        return_rate: return_rate[5],
        risk_rating: risk_rating[5],
        allocation: wbtc_allocation,
        platform_type: 4,
    };
    let sol = Platform {
        id: 7,
        return_rate: return_rate[6],
        risk_rating: risk_rating[6],
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
