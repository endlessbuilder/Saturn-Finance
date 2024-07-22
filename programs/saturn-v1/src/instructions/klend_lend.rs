use crate::{
    account::{SequenceFlag, Treasury},
    constants::*, treasury,
};
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use anchor_spl::token::{Token, TokenAccount, Mint};
use kamino_lending::{
    cpi::accounts::DepositReserveLiquidityAndObligationCollateral,
    program::KaminoLending,
    state::{LendingMarket, Obligation, Reserve},
};

#[derive(Accounts)]
pub struct KaminoLend<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury.treasury_admin.key()
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
        constraint = sequence_flag.flag_kamino && sequence_flag.flag_marginfi && sequence_flag.flag_meteora  == true,
    )]
    pub sequence_flag: Account<'info, SequenceFlag>,

    #[account(mut,
        has_one = lending_market,
        constraint = obligation.load()?.owner == treasury_authority.key(),
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    #[account(mut,
        address = reserve.load()?.liquidity.supply_vault
    )]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = reserve.load()?.collateral.mint_pubkey
    )]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,

    #[account(mut,
        address = reserve.load()?.collateral.supply_vault
    )]
    pub reserve_destination_deposit_collateral: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = reserve.load()?.liquidity.mint_pubkey
    )]
    pub user_source_liquidity: Account<'info, TokenAccount>,

    #[account(mut,
        token::mint = reserve_collateral_mint.key()
    )]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction: AccountInfo<'info>,
    pub klend_program: Program<'info, KaminoLending>,
}

pub fn handle(ctx: Context<KaminoLend>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let amount = (treasury.kamino_allocation * treasury.treasury_value) * (1_000_000) as f64;
    
    // let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        TREASURY_AUTHORITY_SEED.as_ref(),
        &[ctx.bumps.treasury_authority],
    ];

    kamino_lending::cpi::deposit_reserve_liquidity_and_obligation_collateral(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            DepositReserveLiquidityAndObligationCollateral {
                owner: ctx.accounts.treasury_authority.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
                reserve: ctx.accounts.reserve.to_account_info(),
                reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
                reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.to_account_info(),
                reserve_destination_deposit_collateral: ctx
                    .accounts
                    .reserve_destination_deposit_collateral
                    .to_account_info(),
                user_source_liquidity: ctx.accounts.user_source_liquidity.to_account_info(),
                user_destination_collateral: ctx
                    .accounts
                    .user_destination_collateral
                    .to_account_info(),
                // placeholder_user_destination_collateral: Some(ctx.accounts.user_destination_collateral.clone().expect("REASON").to_account_info()),
                token_program: ctx.accounts.token_program.to_account_info(),
                instruction_sysvar_account: ctx
                    .accounts
                    .instruction
                    .to_account_info(),
                // user_destination_collateral: todo!(),
            },
            &[signer_seeds],
        ),
        amount as u64,
    )?;

    Ok(())
}
