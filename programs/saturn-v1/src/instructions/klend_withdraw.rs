use crate::{
    account::{Treasury, SequenceFlag},
    constants::*,
};
use anchor_lang::solana_program::sysvar;
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use anchor_spl::token::{Token, TokenAccount, Mint};
use kamino_lending::{
    cpi::accounts::WithdrawObligationCollateralAndRedeemReserveCollateral,
    program::KaminoLending,
    state::{Obligation, Reserve},
};

#[derive(Accounts)]
pub struct KlendWithdraw<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury_authority.key()
    )]
    signer: Signer<'info>,
    /// CHECK:
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
        constraint = sequence_flag.flag_reallocate == true,
        constraint = sequence_flag.flag_kamino == false,
    )]
    pub sequence_flag: Account<'info, SequenceFlag>,

    #[account(mut,
        token::mint = withdraw_reserve.load()?.liquidity.mint_pubkey
    )]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    /// CHECK: address on account checked
    #[account(address = sysvar::instructions::ID)]
    pub instruction: AccountInfo<'info>,

    /*
     * klend accounts
     */
    pub klend_program: Program<'info, KaminoLending>,
    /// CHECK: devnet demo
    #[account(
        mut,
        has_one = lending_market,
        constraint = obligation.load()?.owner == treasury_authority.key(),
    )]
    pub obligation: AccountLoader<'info, Obligation>,
    /// CHECK: devnet demo
    pub lending_market: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub withdraw_reserve: AccountLoader<'info, Reserve>,

    #[account(mut, address = withdraw_reserve.load()?.collateral.supply_vault)]
    pub reserve_source_collateral: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = withdraw_reserve.load()?.collateral.mint_pubkey)]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut, address = withdraw_reserve.load()?.liquidity.supply_vault)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut,
        token::mint = reserve_collateral_mint.key()
    )]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,
}

pub fn handle(ctx: Context<KlendWithdraw>) -> Result<()> {
    let amount = ctx.accounts.treasury.kamino_lend_assets;
    // let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    kamino_lending::cpi::withdraw_obligation_collateral_and_redeem_reserve_collateral(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            WithdrawObligationCollateralAndRedeemReserveCollateral {
                owner: ctx.accounts.treasury_authority.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
                withdraw_reserve: ctx.accounts.withdraw_reserve.to_account_info(),
                reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
                reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.to_account_info(),
                user_destination_collateral: ctx
                    .accounts
                    .user_destination_collateral
                    .to_account_info(),
                // placeholder_user_destination_collateral: Some(ctx.accounts.user_destination_collateral.clone().expect("REASON").to_account_info()),
                token_program: ctx.accounts.token_program.to_account_info(),
                instruction_sysvar_account: ctx.accounts.instruction.to_account_info(),
                user_destination_liquidity: ctx
                    .accounts
                    .user_destination_liquidity
                    .to_account_info(),
                reserve_source_collateral: ctx.accounts.reserve_source_collateral.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )?;

    ctx.accounts.sequence_flag.flag_kamino = true;

    Ok(())
}
