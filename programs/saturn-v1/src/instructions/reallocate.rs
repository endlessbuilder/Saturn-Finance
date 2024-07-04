use anchor_lang::prelude::*;
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use kamino_lending::program::KaminoLending;
use kamino_lending::state::{LendingMarket, Obligation, Reserve};
use marginfi::program::Marginfi;
use marginfi::state::{
    marginfi_account::MarginfiAccount,
    marginfi_group::{Bank, MarginfiGroup},
};
use crate::account::meteora_account::Partner;
use crate::meteora_utils::{MeteoraProgram, MeteoraUtils};
use meteora::state::Vault;

use crate::utils::*;
use crate::{account::*, constants::*};

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

    //****** kamino accounts ******
    #[account(mut,
        has_one = lending_market,
        constraint = kamino_obligation.load()?.owner == treasury_authority.key(),
    )]
    pub kamino_obligation: AccountLoader<'info, Obligation>,
    pub kamino_lending_market: AccountLoader<'info, LendingMarket>,
    /// CHECK: just authority
    pub kamino_lending_market_authority: AccountInfo<'info>,
    #[account(mut,
        has_one = kamino_lending_market
    )]
    pub kamino_reserve: AccountLoader<'info, Reserve>,
    #[account(mut,
        address = kamino_reserve.load()?.liquidity.supply_vault
    )]
    pub kamino_reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = kamino_reserve.load()?.collateral.mint_pubkey
    )]
    pub kamino_reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut,
        address = kamino_reserve.load()?.collateral.supply_vault
    )]
    pub kamino_reserve_collateral_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::mint = kamino_reserve.load()?.liquidity.mint_pubkey
    )]
    pub kamino_user_source_liquidity: Account<'info, TokenAccount>,
    #[account(mut,
        token::mint = kamino_reserve_collateral_mint.key()
    )]
    pub kamino_user_destination_collateral: Box<Account<'info, TokenAccount>>,
    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction_sysvar_account: AccountInfo<'info>,
    pub kamino_program: Program<'info, KaminoLending>,

    //****** marginfi accounts *******
    pub marginfi_group: AccountLoader<'info, MarginfiGroup>,
    #[account(
        mut,
        constraint = marginfi_account.load()?.group == marginfi_group.key(),
        constraint = marginfi_account.load()?.authority == treasury_authority.key(),
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,
    #[account(
        mut,
        constraint = marginfi_bank.load()?.group == marginfi_group.key(),
    )]
    pub marginfi_bank: AccountLoader<'info, Bank>,
    /// CHECK: marginfi account
    #[account(mut)]
    pub marginfi_bank_liquidity_vault: Account<'info, TokenAccount>,
    /// CHECK: Seed constraint check
    #[account(mut)]
    pub marginfi_bank_liquidity_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub marginfi_user_liquidity: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub marginfi_program: Program<'info, Marginfi>,

    //******** meteora accounts *********
    /// partner info CHECK:
    #[account(mut, has_one = vault)]
    pub meteora_partner: Box<Account<'info, Partner>>,
    /// CHECK:
    #[account(mut)]
    pub meteora_vault: Box<Account<'info, Vault>>,
    /// CHECK:
    #[account(mut)]
    pub meteora_token_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub meteora_vault_lp_mint: Box<Account<'info, Mint>>,
    /// treasury token CHECK:
    #[account(mut)]
    pub meteora_treasury_token: UncheckedAccount<'info>,
    /// treasury lp CHECK:
    #[account(mut, constraint = treasury_lp.owner == treasury_authority.key())] //mint to account of treasury PDA
    pub meteora_treasury_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub meteora_vault_program: Program<'info, MeteoraProgram>,


    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[allow(unused_variables)]
pub fn handle(
    ctx: Context<ReAllocate>,
    treasur: [[u64; 5]; 7],
    platform_allocation: [u64; 4],
) -> Result<()> {
    Ok(())
}
