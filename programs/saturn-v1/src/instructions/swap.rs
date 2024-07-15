use std::str::FromStr;

use crate::{
    account::{Escrow, Treasury, UserStakeAccount},
    constants::*,
    error::*,
    jupiter_utils::*,
    treasury,
};
use anchor_lang::{
    accounts::unchecked_account,
    prelude::*,
    solana_program::{entrypoint::ProgramResult, instruction::Instruction, program::invoke_signed},
    system_program,
};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{self, get_feed_id_from_hex, PriceUpdateV2};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury_authority.key()
    )]
    signer: Signer<'info>,
    /// CHECK: this is pda
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

    /// CHECK: This may not be initialized yet.
    #[account(mut, token::mint = Pubkey::from_str(WBTC_MINT).unwrap())]
    pub wbtc_treasury_token_account: Account<'info, TokenAccount>,

    pub wbtc_mint: Account<'info, Mint>,

    /// CHECK: This may not be initialized yet.
    #[account(mut, token::mint = Pubkey::from_str(USDT_MINT).unwrap())]
    pub usdt_treasury_token_account: Account<'info, TokenAccount>,

    pub usdt_mint: Account<'info, Mint>,

    /// CHECK: This may not be initialized yet.
    #[account(mut, token::mint = Pubkey::from_str(USDC_MINT).unwrap())]
    pub usdc_treasury_token_account: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    pub sol_mint: Account<'info, Mint>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub jupiter_program: Program<'info, Jupiter>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Swap>, data: Vec<u8>, from_amount: u64) -> Result<()> {
    let authority_bump = ctx.bumps.treasury_authority;
    let treasury = &mut ctx.accounts.treasury;
    let treasury_authority = &mut ctx.accounts.treasury_authority;
    let usdc_token_account = &mut ctx.accounts.usdc_treasury_token_account;
    let usdt_token_account = &mut ctx.accounts.usdt_treasury_token_account;
    let wbtc_token_account = &mut ctx.accounts.wbtc_treasury_token_account;

    let price_update = &mut ctx.accounts.price_update;
    let maximum_age: u64 = 30;
    let usdc2usdt_amount = (treasury.usdc_allocation * treasury.treasury_value as f64) as u64
        - usdc_token_account.amount / 1_000_000;

    let wbtc_feed_id: [u8; 32] = get_feed_id_from_hex(WBTC_PRICE_ID)?;
    let wbtc_price =
        price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &wbtc_feed_id)?;
    let wbtc_val = wbtc_token_account.amount * u64::try_from(wbtc_price.price).unwrap()
        / 10u64.pow(u32::try_from(-wbtc_price.exponent).unwrap());

    let usdc2wbtc_amount =
        (treasury.wbtc_allocation * treasury.treasury_value as f64) as u64 - wbtc_val;

    let sol_feed_id: [u8; 32] = get_feed_id_from_hex(SOL_PRICE_ID)?;
    let sol_price =
        price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &sol_feed_id)?;
    let sol_val = (treasury_authority.lamports() / 1_000_000_000)
        * u64::try_from(sol_price.price).unwrap()
        / 10u64.pow(u32::try_from(-sol_price.exponent).unwrap());

    let usdc2sol_amount = ((treasury.sol_allocation * treasury.treasury_value as f64) as u64
        - sol_val)
        / (u64::try_from(sol_price.price).unwrap()
            / 10u64.pow(u32::try_from(-sol_price.exponent).unwrap()));
    // let from_amount = 1000000;

    // if ctx.accounts.to_mint.key() == wsol_mint {
    // create_wsol_token_idempotent(
    //     ctx.accounts.treasury_authority.clone(),
    //     ctx.accounts.to_treasury_token_account.clone(),
    //     ctx.accounts.sol_mint.clone(),
    //     ctx.accounts.token_program.clone(),
    //     ctx.accounts.system_program.clone(),
    //     authority_bump,
    //     to_treasury_token_account_bump,
    //     0u64,
    // )?;
    // }
    msg!("Swap on Jupiter");

    let authority_bump_seeds = [authority_bump];
    let signer_seeds: &[&[&[u8]]] = &[&[
        TREASURY_AUTHORITY_SEED.as_bytes(),
        authority_bump_seeds.as_ref(),
    ]];
    swap_on_jupiter(
        ctx.remaining_accounts,
        ctx.accounts.jupiter_program.clone(),
        data,
        signer_seeds,
        ctx.accounts.treasury_authority.key,
    )?;

    // if ctx.accounts.to_mint.key() == wsol_mint {
    // close_treasury_wsol(
    //     ctx.accounts.treasury_authority.clone(),
    //     ctx.accounts.to_treasury_token_account.clone(),
    //     ctx.accounts.token_program.clone(),
    //     &[authority_bump],
    // )?;
    // }

    Ok(())
}
