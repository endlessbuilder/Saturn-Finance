use anchor_lang::prelude::*;
use anchor_spl::token::{self, mint_to, MintTo, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::pubkey::Pubkey;

use crate::{
    account::{Escrow, Treasury},
    constants::*,
    error::*,
    utils::*,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ApplyBondArgs {
    pub token_amount: u64,
    pub spot_price: u64,
}

#[derive(Accounts)]
#[instruction(args: ApplyBondArgs)]
pub struct ApplyBond<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(zero)]
    pub escrow: AccountLoader<'info, Escrow>,

    #[account(
        mut,
        constraint = creater_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = creater_token_account.owner == *admin.key,
    )]
    pub creater_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = dest_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = dest_token_account.owner == *treasury.to_account_info().key,
    )]
    pub dest_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = dest_stf_account.mint == *stf_token_mint.to_account_info().key,
        constraint = dest_stf_account.owner == *treasury.to_account_info().key,
    )]
    pub dest_stf_account: Account<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_mint_address: AccountInfo<'info>,
    #[account(
        mut,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stf_token_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(
    ctx: Context<ApplyBond>,
    token_amount: u64,
    spot_price: u64,
) -> Result<()> {
    let mut escrow = ctx.accounts.escrow.load_init()?;
    let treasury = &mut ctx.accounts.treasury;
    msg!("apply_bond");
    let src_account_info = &mut &ctx.accounts.creater_token_account;
    let dest_account_info = &mut &ctx.accounts.dest_token_account;
    let dest_stf_account = &mut &ctx.accounts.dest_stf_account;
    let token_program = &mut &ctx.accounts.token_program;
    let stf_token_mint = &mut &ctx.accounts.stf_token_mint;
    let mint_pubkey = &mut &ctx.accounts.token_mint_address.key().to_string();

    let price_update = &mut ctx.accounts.price_update;

    assert!(
        stf_token_mint.key().to_string().as_str() == STF_MINT,
        "STF_TOKEN_MINT ERROR"
    );

    let maximum_age: u64 = 30;
    let feed_id: [u8; 32];
    //     get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?;
    if mint_pubkey.as_str() == SOL_MINT {
        feed_id = get_feed_id_from_hex(SOL_PRICE_ID)?;
        sol_transfer_user(
            admin.to_account_info(),
            treasury.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            token_amount,
        )?;
    } else if mint_pubkey.as_str() == USDC_MINT {
        feed_id = get_feed_id_from_hex(USDC_PRICE_ID)?;
        let cpi_accounts = Transfer {
            from: src_account_info.to_account_info().clone(),
            to: dest_account_info.to_account_info().clone(),
            authority: ctx.accounts.admin.to_account_info().clone(),
        };
        token::transfer(
            CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
            token_amount,
        )?;
    } else if mint_pubkey.as_str() == BONK_MINT {
        feed_id = get_feed_id_from_hex(BONK_PRICE_ID)?;
        let cpi_accounts = Transfer {
            from: src_account_info.to_account_info().clone(),
            to: dest_account_info.to_account_info().clone(),
            authority: ctx.accounts.admin.to_account_info().clone(),
        };
        token::transfer(
            CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
            token_amount,
        )?;
    } else {
        return Err(BondError::TokenMintError.into());
    }

    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

    msg!(
        "The price is ({} ± {}) * 10^{}",
        price.price,
        price.conf,
        price.exponent
    );

    let total_price = price.price * token_amount as i64; // 10 ** token_decimal 
    let backing_price = treasury.treasury_value / treasury.token_minted; // this is per lamport stf
    let spot_price = spot_price;    

    let diff = (spot_price - backing_price) * 100 / backing_price;
    let deduction;
    if diff < 25 {
        deduction = 20;
    } else if diff < 50 {
        deduction = 35;
    } else if diff < 100 {
        deduction = 50;
    } else {
        deduction = 70;
    }
    let bond_price = backing_price + (spot_price - backing_price) * deduction / 100;

    let num_token_to_mint =
        (bond_price - backing_price) * total_price as u64 / backing_price / bond_price; // should multiply the decimal of the staking token
    let num_token_to_redeem = backing_price * total_price as u64 / backing_price / bond_price; // should multiply the decimal of the staking token

    treasury.token_minted += num_token_to_mint;
    treasury.staking_index += num_token_to_mint / treasury.token_staked;
    //
    treasury.treasury_value += total_price as u64;

    let cpi_accounts = MintTo {
        mint: stf_token_mint.to_account_info().clone(),
        to: dest_stf_account.to_account_info().clone(),
        authority: ctx.accounts.treasury.to_account_info().clone(),
    };
    let seeds = &[TREASURY_SEED.as_bytes(), &[ctx.bumps.treasury]];
    let signer = &[&seeds[..]];
    
    token::mint_to(
        CpiContext::new_with_signer(
            token_program.clone().to_account_info(),
            cpi_accounts,
            signer,
        ),
        num_token_to_mint,
    )?;

    let timestamp = Clock::get()?.unix_timestamp;
    escrow.creator = ctx.accounts.admin.key();
    escrow.token_mint = ctx.accounts.token_mint_address.key();
    escrow.token_amount = token_amount;
    escrow.num_token_to_redeem = num_token_to_redeem;
    escrow.start_timestamp = timestamp;
    escrow.end_timestamp = timestamp + 60 * 60 * 24 * 14; // 14days
    escrow.is_finished = 1;
    Ok(())
}
