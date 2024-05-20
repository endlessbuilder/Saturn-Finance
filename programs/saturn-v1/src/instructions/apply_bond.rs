use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::pubkey::Pubkey;

use crate::{
    account::{Escrow, Treasury},
    constants::*,
    error::*,
    utils::*
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

    pub price_update: Account<'info, PriceUpdateV2>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_mint_address: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<ApplyBond>, args: ApplyBondArgs) -> Result<()> {
    let mut escrow = ctx.accounts.escrow.load_init()?;
    let treasury = &mut ctx.accounts.treasury;
    msg!("apply_bond");
    let src_account_info = &mut &ctx.accounts.creater_token_account;
    let dest_account_info = &mut &ctx.accounts.dest_token_account;
    let mint_pubkey = &mut &ctx.accounts.token_mint_address.key().to_string();

    let price_update = &mut ctx.accounts.price_update;
    let maximum_age: u64 = 30;
    let feed_id: [u8; 32];
    //     get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?;
    if mint_pubkey.as_str() == SOL_MINT {
        feed_id = get_feed_id_from_hex(SOL_PRICE_ID)?;
        sol_transfer_user(
            src_account_info.to_account_info(),
            dest_account_info.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            args.token_amount,
        )?;
    } else if mint_pubkey.as_str() == USDC_MINT {
        feed_id = get_feed_id_from_hex(USDC_PRICE_ID)?;
    } else if mint_pubkey.as_str() == BONK_MINT {
        feed_id = get_feed_id_from_hex(BONK_PRICE_ID)?;
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

    let total_price = price.price * args.token_amount as i64;
    let backing_price = treasury.treasury_value / treasury.token_minted;
    let spot_price = args.spot_price;

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
    let bond_price = backing_price + diff * deduction / 100;

    let num_token_to_mint = (bond_price - backing_price) * total_price as u64 / backing_price / bond_price; // should multiply the decimal of the staking token
    let num_token_to_redeem = backing_price * total_price as u64 / backing_price / bond_price; // should multiply the decimal of the staking token

    msg!("token_to_mint{}", num_token_to_mint);

    escrow.creator = ctx.accounts.admin.key(); 
    escrow.token_mint = ctx.accounts.token_mint_address.key();
    escrow.token_amount = args.token_amount;
    escrow.num_token_to_redeem = num_token_to_redeem;
    Ok(())
}
